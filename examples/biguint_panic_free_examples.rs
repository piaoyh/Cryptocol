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
    biguint_panic_free_quick_start();
    biguint_panic_free_add();
    biguint_panic_free_sub();
    biguint_panic_free_mul();
    biguint_div();
    biguint_rem();
    biguint_panic_free_exponentiation_logarithm();
    biguint_panic_free_next_multiple();
    biguint_panic_free_misc();
}

fn biguint_panic_free_quick_start()
{
    println!("biguint_panic_free_quick_start");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    let one = U256::one();
    let res = a_biguint.panic_free_modular_add(&one, &m);
    println!("{} + {} = {}", a_biguint, one, res);
    assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");

    let a_biguint = U256::from_uint(10_u8);
    let exp = U256::from_uint(30_u8);
    let res = a_biguint.panic_free_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");

    let a_biguint = U256::max();
    let num = U256::from(586478_u32);
    let multiple = a_biguint.panic_free_next_multiple_of(&num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "448670");

    let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "27");
    println!("---------------------------");
}

fn biguint_panic_free_add()
{
    biguint_panic_free_modular_add_uint();
    biguint_panic_free_modular_add_assign_uint();
    biguint_panic_free_modular_add();
    biguint_panic_free_modular_add_assign();
}

fn biguint_panic_free_modular_add_uint()
{
    println!("biguint_panic_free_modular_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_add()
{
    println!("biguint_panic_free_modular_add");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    // Normal case 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    let one = U256::one();
    let res = a_biguint.panic_free_modular_add(&one, &m);
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
    let res = a_biguint.panic_free_modular_add(&two, &m);
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
    let res = a_biguint.panic_free_modular_add(&three, &m);
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
    let res = a_biguint.panic_free_modular_add(&three, &m);
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
    let res = a_biguint.panic_free_modular_add(&three, &m);
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
    let res = a_biguint.panic_free_modular_add(&three, &m);
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
    let res = a_biguint.panic_free_modular_add(&zero, &m);
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
    let res = a_biguint.panic_free_modular_add(&multiple_of_modulo, &m);
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
    let res = a_biguint.panic_free_modular_add(&zero, &m);
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
    let res = a_biguint.panic_free_modular_add(&zero, &m);
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
    let res = a_biguint.panic_free_modular_add(&multiple_of_modulo, &m);
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
    let res = a_biguint.panic_free_modular_add(&multiple_of_modulo, &m);
    println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo = 0
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::zero();
    let rhs = U256::from_uint(3_u8);
    let res = a_biguint.panic_free_modular_add(&rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo = 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::one();
    let rhs = U256::from_uint(3_u8);
    let res = a_biguint.panic_free_modular_add(&rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Collective Example for modulo == 0 or 1
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_add(&rhs, &m);
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

fn biguint_panic_free_modular_add_assign()
{
    println!("biguint_panic_free_modular_add_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

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
    a_biguint.panic_free_modular_add_assign(&one, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::one(), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&two, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(2_u8), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&three, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_add_assign(&three, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&three, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&three, &m);
    println!("After a_biguint.panic_free_modular_add_assign(U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&three, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&zero, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&multiple_of_modulo, &m);
    println!("After a_biguint.panic_free_modular_add_assign(& U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
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
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    a_biguint.panic_free_modular_add_assign(&zero, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
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
    a_biguint.panic_free_modular_add_assign(&zero, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
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
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    a_biguint.panic_free_modular_add_assign(&multiple_of_modulo, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // modulo == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::zero();
    let rhs = U256::one();
    a_biguint.panic_free_modular_add_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::one(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    let m = U256::one();
    let rhs = U256::one();
    a_biguint.panic_free_modular_add_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign(&U256::one(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Collective Example for modulo == 0 or 1
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
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
            
                a_biguint.panic_free_modular_add_assign(&rhs, &m);
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


fn biguint_panic_free_sub()
{
    biguint_panic_free_modular_sub_uint();
    biguint_panic_free_modular_sub_assign_uint();
    biguint_panic_free_modular_sub();
    biguint_panic_free_modular_sub_assign();
}

fn biguint_panic_free_modular_sub_uint()
{
    println!("biguint_panic_free_modular_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_sub()
{
    println!("biguint_panic_free_modular_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    // Normal Case 1
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let one = U256::one();
    let res = a_biguint.panic_free_modular_sub(&one, &m);
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
    let res = a_biguint.panic_free_modular_sub(&one, &m);
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
    let res = a_biguint.panic_free_modular_sub(&three, &m);
    println!("{} - {} = {} (mod {})", a_biguint, three, res, m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
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
    let rhs = U256::from_uint(50_u8);
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::one();
    let rhs = U256::from_uint(50_u8);
    let res = a_biguint.panic_free_modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Collective Examples for modulo == 0 or 1
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_sub(&rhs, &m);
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

fn biguint_panic_free_modular_sub_assign()
{
    println!("biguint_panic_free_modular_sub_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // modulo == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::zero();
    let rhs = U256::from_uint(250_u8);
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // modulo == 1
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::one();
    let rhs = U256::from_uint(250_u8);
    a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Collective Examples for modulo == 0 or 1
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
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
            
                a_biguint.panic_free_modular_sub_assign(&rhs, &m);
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


fn biguint_panic_free_mul()
{
    biguint_panic_free_modular_mul_uint();
    biguint_panic_free_modular_mul_assign_uint();
    biguint_panic_free_modular_mul();
    biguint_panic_free_modular_mul_assign();
}

fn biguint_panic_free_modular_mul_uint()
{
    println!("biguint_panic_free_modular_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_mul()
{
    println!("biguint_panic_free_modular_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // Normal case 1
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mul_biguint = UU32::from_uint(5_u8);
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
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
    let mul_biguint = UU32::from_uint(248_u8);
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
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
    let mul_biguint = UU32::from_uint(248_u8);
    let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // Collective Example for modulo == 0 or 1
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_mul(&rhs, &m);
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

fn biguint_panic_free_modular_mul_assign()
{
    println!("biguint_panic_free_modular_mul_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
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
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::zero();
    let mul_biguint = UU32::from_uint(248_u8);
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::one();
    let mul_biguint = UU32::from_uint(248_u8);
    a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Collective Example for modulo == 0 or 1
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [U256::zero(), U256::from_uint(3_u8), U256::from_uint(50_u8)]
        {
            for m in [U256::zero(), U256::one()]
            {
                let mut a_biguint = a.clone();
                println!("Originally, a_biguint = {}", a);
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), false);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);

                a_biguint.panic_free_modular_mul_assign(&rhs, &m);
                println!("After b_biguint.panic_free_modular_mul_assign(&rhs, &m), a_biguint = {}", a_biguint);
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


fn biguint_div()
{
    biguint_panic_free_divide_fully_uint();
    biguint_panic_free_div_uint();
    biguint_panic_free_div_assign_uint();
    biguint_panic_free_modular_div_uint();
    biguint_panic_free_modular_div_assign_uint();
    biguint_panic_free_divide_fully();
    biguint_panic_free_div();
    biguint_panic_free_div_assign();
    biguint_panic_free_modular_div();
    biguint_panic_free_modular_div_assign();
}

fn biguint_panic_free_divide_fully_uint()
{
    println!("biguint_panic_free_divide_fully_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_div_uint()
{
    println!("biguint_panic_free_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_div_uint()
{
    println!("biguint_panic_free_modular_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_divide_fully()
{
    println!("biguint_panic_free_divide_fully");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u16);

    // Normal case 1
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let (quotient, remainder) = dividend.panic_free_divide_fully(&divisor);
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

    // Normal case 2
    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let (quotient, remainder) = dividend.panic_free_divide_fully(&divisor);
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

    // dividend != 0 and divisor == 0
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::zero();
    let (quotient, remainder) = dividend.panic_free_divide_fully(&divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient, UU32::max());
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), true);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // dividend == 0 and divisor == 0
    let dividend = UU32::zero();
    let divisor = UU32::zero();
    let (quotient, remainder) = dividend.panic_free_divide_fully(&divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_div()
{
    println!("biguint_panic_free_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.panic_free_div(&divisor);
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
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.panic_free_div(&divisor);
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
    let divisor = U256::zero();
    let quotient = dividend.panic_free_div(&divisor);
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
    let divisor = U256::zero();
    let quotient = dividend.panic_free_div(&divisor);
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

fn biguint_panic_free_div_assign()
{
    println!("biguint_panic_free_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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

    let divisor = UU32::from_uint(87_u8);
    a_biguint.panic_free_div_assign(&divisor);
    println!("After a_biguint.panic_free_div_assign(&divisor),\na_biguint = {}", a_biguint);
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

    let divisor = UU32::from_uint(87_u8);
    a_biguint.panic_free_div_assign(&divisor);
    println!("After a_biguint.panic_free_div_assign(&divisor),\na_biguint = {}", a_biguint);
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

    let divisor = UU32::zero();
    a_biguint.panic_free_div_assign(&divisor);
    println!("After a_biguint.panic_free_div_assign(&divisor),\na_biguint = {}", a_biguint);
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

    let divisor = UU32::zero();
    a_biguint.panic_free_div_assign(&divisor);
    println!("After a_biguint.panic_free_div_assign(&divisor),\na_biguint = {}", a_biguint);
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

fn biguint_panic_free_modular_div()
{
    println!("biguint_panic_free_modular_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);
    
    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::zero();
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::one();
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::zero();
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::one();
    let quotient = dividend.panic_free_modular_div(&divisor, &modulo);
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
        let op2 = U256::zero();
        let res = op1.panic_free_modular_div(&op2, &modulo);
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
            let rhs = U256::zero();
            let res = dividend.panic_free_modular_div(&rhs, &modulo);
            println!("{} / {} = {} (mod {})", dividend, rhs, res, modulo);
            assert_eq!(res.to_string(), "0");
            assert_eq!(res.is_overflow(), false);
            assert_eq!(res.is_underflow(), false);
            assert_eq!(res.is_divided_by_zero(), true);
            assert_eq!(res.is_infinity(), true);
            assert_eq!(res.is_undefined(), true);
            assert_eq!(res.is_left_carry(), false);
            assert_eq!(res.is_right_carry(), false);

            for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
            {
                let res = dividend.panic_free_modular_div(&divisor, &modulo);
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

fn biguint_panic_free_modular_div_assign()
{
    println!("biguint_panic_free_modular_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

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

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
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

    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(128_u8);
    let modulo = U256::zero();
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(128_u8);
    let modulo = U256::one();
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::zero();
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::one();
    a_biguint.panic_free_modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

        let divisor = U256::zero();
        dividend.panic_free_modular_div_assign(&divisor, &modulo);
        println!("After a_biguint.panic_free_modular_div_assign({}, {}), op1 = {}", divisor, modulo, dividend);
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

            let divisor = U256::zero();
            dividend.panic_free_modular_div_assign(&divisor, &modulo);
            println!("After op1.panic_free_modular_div_assign({}, {}), dividend = {}", divisor, modulo, dividend);
            assert_eq!(dividend.to_string(), "0");
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_divided_by_zero(), true);
            assert_eq!(dividend.is_infinity(), true);
            assert_eq!(dividend.is_undefined(), true);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
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
    
                dividend.panic_free_modular_div_assign(&divisor, &modulo);
                println!("After dividend.panic_free_modular_div_assign({}, {}), dividend = {}", divisor, modulo, dividend);
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

fn biguint_rem()
{
    biguint_panic_free_rem_uint();
    biguint_panic_free_rem_assign_uint();
    biguint_panic_free_modular_rem_uint();
    biguint_panic_free_modular_rem_assign_uint();
    biguint_panic_free_rem();
    biguint_panic_free_rem_assign();
    biguint_panic_free_modular_rem();
    biguint_panic_free_modular_rem_assign();
}

fn biguint_panic_free_rem_uint()
{
    println!("biguint_panic_free_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_rem_uint()
{
    println!("biguint_panic_free_modular_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_rem()
{
    println!("biguint_panic_free_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // Normal case 1
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.panic_free_rem(&divisor);
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
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.panic_free_rem(&divisor);
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
    let divisor = UU32::zero();
    let remainder = dividend.panic_free_rem(&divisor);
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
    let divisor = UU32::zero();
    let remainder = dividend.panic_free_rem(&divisor);
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

fn biguint_panic_free_rem_assign()
{
    println!("biguint_panic_free_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // Normal case 1
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
    a_biguint.panic_free_rem_assign(&divisor);
    println!("After a_biguint.panic_free_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::from_uint(87_u8);
    a_biguint.panic_free_rem_assign(&divisor);
    println!("After a_biguint.panic_free_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    
    let divisor = U256::zero();
    a_biguint.panic_free_rem_assign(&divisor);
    println!("After a_biguint.panic_free_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    
    let divisor = U256::zero();
    a_biguint.panic_free_rem_assign(&divisor);
    println!("After a_biguint.panic_free_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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

fn biguint_panic_free_modular_rem()
{
    println!("biguint_panic_free_modular_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::zero();
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::zero();
    let modulo = U256::one();
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::zero();
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::one();
    let remainder = dividend.panic_free_modular_rem(&divisor, &modulo);
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
        let op2 = U256::zero();
        let res = op1.panic_free_modular_rem(&op2, &modulo);
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
            let rhs = U256::zero();
            let res = dividend.panic_free_modular_rem(&rhs, &modulo);
            println!("{} % {} = {} (mod {})", dividend, rhs, res, modulo);
            assert_eq!(res.to_string(), "0");
            assert_eq!(res.is_overflow(), false);
            assert_eq!(res.is_underflow(), false);
            assert_eq!(res.is_divided_by_zero(), true);
            assert_eq!(res.is_infinity(), false);
            assert_eq!(res.is_undefined(), true);
            assert_eq!(res.is_left_carry(), false);
            assert_eq!(res.is_right_carry(), false);

            for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
            {
                let res = dividend.panic_free_modular_rem(&divisor, &modulo);
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

fn biguint_panic_free_modular_rem_assign()
{
    println!("biguint_panic_free_modular_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

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
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
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

    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::zero();
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::from_uint(128_u8);
    let modulo = U256::zero();
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::from_uint(128_u8);
    let modulo = U256::one();
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::zero();
    let modulo = U256::zero();
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::zero();
    let modulo = U256::one();
    a_biguint.panic_free_modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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

        let divisor = U256::zero();
        dividend.panic_free_modular_rem_assign(&divisor, &modulo);
        println!("After a_biguint.panic_free_modular_rem_assign({}, {}), op1 = {}", divisor, modulo, dividend);
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

            let divisor = U256::zero();
            dividend.panic_free_modular_rem_assign(&divisor, &modulo);
            println!("After op1.panic_free_modular_rem_assign({}, {}), dividend = {}", divisor, modulo, dividend);
            assert_eq!(dividend.to_string(), "0");
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_divided_by_zero(), true);
            assert_eq!(dividend.is_infinity(), false);
            assert_eq!(dividend.is_undefined(), true);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
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
    
                dividend.panic_free_modular_rem_assign(&divisor, &modulo);
                println!("After dividend.panic_free_modular_rem_assign({}, {}), dividend = {}", divisor, modulo, dividend);
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

fn biguint_panic_free_exponentiation_logarithm()
{
    biguint_panic_free_pow_uint();
    biguint_panic_free_pow_assign_uint();
    biguint_panic_free_modular_pow_uint();
    biguint_panic_free_modular_pow_assign_uint();
    biguint_panic_free_iroot_uint();
    biguint_panic_free_iroot_assign_uint();
    biguint_panic_free_ilog_uint();
    biguint_panic_free_ilog_assign_uint();
    biguint_panic_free_pow();
    biguint_panic_free_pow_assign();
    biguint_panic_free_modular_pow();
    biguint_panic_free_modular_pow_assign();
    biguint_panic_free_iroot();
    biguint_panic_free_iroot_assign();
    biguint_panic_free_ilog();
    biguint_panic_free_ilog_assign();
    biguint_panic_free_ilog2();
    biguint_panic_free_ilog2_assign();
    biguint_panic_free_ilog10();
    biguint_panic_free_ilog10_assign();
}
fn biguint_panic_free_pow_uint()
{
    println!("biguint_panic_free_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u8;
    let res = a_biguint.panic_free_pow_uint(exp);
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
    let exp = 100_u8;
    let res = a_biguint.panic_free_pow_uint(exp);
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
    let res = a_biguint.panic_free_pow_uint(exp);
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
    let res = a_biguint.panic_free_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::zero();
    let exp = 0_u8;
    let res = a_biguint.panic_free_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_pow_assign_uint()
{
    println!("biguint_panic_free_pow_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 10_u8;
    a_biguint.panic_free_pow_assign_uint(exp);
    println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "10000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10000000000_u64);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 10_u8;
    a_biguint.panic_free_pow_assign_uint(exp);
    println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.panic_free_pow_assign_uint(exp);
    println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.panic_free_pow_assign_uint(exp);
    println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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

    let exp = 0_u8;
    a_biguint.panic_free_pow_assign_uint(exp);
    println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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

fn biguint_panic_free_modular_pow_uint()
{
    println!("biguint_panic_free_modular_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // Noraml case 1
    let a_biguint = U256::from_uint(10_u8);
    let exp = 30_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::from_uint(10_u8);
    let exp = 100_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp == 0 and modulo != 0
    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp == multiple of modulo and modulo != 0
    let a_biguint = UU32::from_uint(10_u8);
    let exp = 2000_u16;
    let modulo = U256::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp != 0 and modulo != 0
    let a_biguint = UU32::zero();
    let exp = 30_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let a_biguint = U256::from_uint(3000_u16);
    let exp = 30_u8;
    let modulo = U256::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let a_biguint = UU32::zero();
    let exp = 0_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let a_biguint = U256::zero();
    let exp = 2000_u16;
    let modulo = U256::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let a_biguint = U256::from_uint(3000_u16);
    let exp = 0_u8;
    let modulo = U256::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let a_biguint = U256::from_uint(3000_u16);
    let exp = 2000_u16;
    let modulo = U256::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 0
    let a_biguint = U256::from_uint(10_u8);
    let exp = 100_u8;
    let modulo = U256::zero();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 1
    let a_biguint = U256::from_uint(10_u8);
    let exp = 100_u8;
    let modulo = U256::one();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo == 0
    let a_biguint = U256::zero();
    let exp = 0_u8;
    let modulo = U256::zero();
    let res = a_biguint.panic_free_modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // collectively
    for modulo in [U256::zero(), U256::one()]
    {
        for lhs in [U256::zero(), U256::from_uint(50_u8)]
        {
            for rhs in [0_u8, 50_u8]
            {
                let res = lhs.panic_free_modular_pow_uint(rhs, &modulo);
                println!("{} ** {} = {} (mod {})", lhs, rhs, res, modulo);
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

fn biguint_panic_free_modular_pow_assign_uint()
{
    println!("biguint_panic_free_modular_pow_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // Noraml case 1
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
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::from_uint(1000000000000000000000000000000_u128);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 100_u8;
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp == 0 and modulo != 0
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
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp == multiple of modulo and modulo != 0
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp != 0 and modulo != 0
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
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let mut a_biguint = UU32::from_uint(300_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 150_u8;
    let modulo = U256::from_uint(50_u8);
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 0
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
    let modulo = U256::zero();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 1
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
    let modulo = U256::one();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    let modulo = U256::zero();
    a_biguint.panic_free_modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // collectively
    for modulo in [U256::zero(), U256::one()]
    {
        for lhs in [U256::zero(), U256::from_uint(50_u8)]
        {
            for rhs in [0_u8, 50_u8]
            {
                let mut lhs = lhs.clone();
                println!("Originally, lhs = {}", lhs);
                assert_eq!(lhs.is_overflow(), false);
                assert_eq!(lhs.is_underflow(), false);
                assert_eq!(lhs.is_infinity(), false);
                assert_eq!(lhs.is_undefined(), false);
                assert_eq!(lhs.is_divided_by_zero(), false);
                assert_eq!(lhs.is_left_carry(), false);
                assert_eq!(lhs.is_right_carry(), false);

                lhs.panic_free_modular_pow_assign_uint(rhs, &modulo);
                println!("After lhs.panic_free_modular_pow_assign_uint({}, {}), lhs = {}", rhs, modulo, lhs);
                assert_eq!(lhs.to_string(), "0");
                assert_eq!(lhs.is_overflow(), false);
                assert_eq!(lhs.is_underflow(), false);
                assert_eq!(lhs.is_infinity(), false);
                assert_eq!(lhs.is_undefined(), true);
                assert_eq!(lhs.is_divided_by_zero(), false);
                assert_eq!(lhs.is_left_carry(), false);
                assert_eq!(lhs.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_iroot_uint()
{
    println!("biguint_panic_free_iroot_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 8_u8;
    let res = a_biguint.panic_free_iroot_uint(exp);
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
    let res = a_biguint.panic_free_iroot_uint(exp);
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
    let res = a_biguint.panic_free_iroot_uint(exp);
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
    let res = a_biguint.panic_free_iroot_uint(exp);
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
    let res = a_biguint.panic_free_iroot_uint(exp);
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
    let res = a_biguint.panic_free_iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 0_u8;
    let res = a_biguint.panic_free_iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let exp = 0_u8;
    let res = a_biguint.panic_free_iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let exp = 0_u8;
    let res = a_biguint.panic_free_iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_iroot_assign_uint()
{
    println!("biguint_panic_free_iroot_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    let exp = 0_u8;
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let exp = 0_u8;
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let exp = 0_u8;
    a_biguint.panic_free_iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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

fn biguint_panic_free_ilog_uint()
{
    println!("biguint_panic_free_ilog_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    let res = a_biguint.panic_free_ilog_uint(base);
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
    let res = a_biguint.panic_free_ilog_uint(base);
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
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 0_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = 6_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = 0_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = 1_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = 0_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = 1_u8;
    let res = a_biguint.panic_free_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_ilog_assign_uint()
{
    println!("biguint_panic_free_ilog_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

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
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
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
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
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
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    let base = 0_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 1_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 6_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 0_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 1_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 0_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = 1_u8;
    a_biguint.panic_free_ilog_assign_uint(base);
    println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
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

fn biguint_panic_free_pow()
{
    println!("biguint_panic_free_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let res = a_biguint.panic_free_pow(&exp);
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
    let res = a_biguint.panic_free_pow(&exp);
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
    let res = a_biguint.panic_free_pow(&exp);
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
    let res = a_biguint.panic_free_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 0 ** 0
    let a_biguint = U256::zero();
    let exp = U256::zero();
    let res = a_biguint.panic_free_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_pow_assign()
{
    println!("biguint_panic_free_pow_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u16);

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
    a_biguint.panic_free_pow_assign(&exp);
    println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.panic_free_pow_assign(&exp);
    println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.panic_free_pow_assign(&exp);
    println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.panic_free_pow_assign(&exp);
    println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic example
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    a_biguint.panic_free_pow_assign(&exp);
    println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(),  true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_modular_pow()
{
    println!("biguint_panic_free_modular_pow()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    // Noraml case 1
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let modulo = UU32::halfmax();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let modulo = UU32::halfmax();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp == 0 and modulo != 0
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::zero();
    let modulo = UU32::halfmax();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp == multiple of modulo and modulo != 0
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(2000_u16);
    let modulo = UU32::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp != 0 and modulo != 0
    let a_biguint = UU32::zero();
    let exp = UU32::from_uint(30_u8);
    let modulo = UU32::halfmax();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let a_biguint = UU32::from_uint(3000_u16);
    let exp = UU32::from_uint(30_u8);
    let modulo = UU32::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let a_biguint = UU32::zero();
    let exp = UU32::zero();
    let modulo = UU32::halfmax();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let a_biguint = UU32::zero();
    let exp = UU32::from_uint(2000_u16);
    let modulo = UU32::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let a_biguint = UU32::from_uint(3000_u16);
    let exp = UU32::zero();
    let modulo = UU32::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let a_biguint = UU32::from_uint(3000_u16);
    let exp = UU32::from_uint(2000_u16);
    let modulo = UU32::from_uint(1000_u16);
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 0
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let modulo = UU32::zero();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 1
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let modulo = UU32::one();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo == 0
    let a_biguint = UU32::zero();
    let exp = UU32::zero();
    let modulo = UU32::zero();
    let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // collectively
    for modulo in [UU32::zero(), UU32::one()]
    {
        for lhs in [UU32::zero(), UU32::from_uint(50_u8)]
        {
            for rhs in [UU32::zero(), UU32::from_uint(50_u8)]
            {
                let res = lhs.panic_free_modular_pow(&rhs, &modulo);
                println!("{} ** {} = {} (mod {})", lhs, rhs, res, modulo);
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

fn biguint_panic_free_modular_pow_assign()
{
    println!("biguint_panic_free_modular_pow_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    // Noraml case 1
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::from_uint(1000000000000000000000000000000_u128);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp == 0 and modulo != 0
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp == multiple of modulo and modulo != 0
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(200_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp != 0 and modulo != 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let mut a_biguint = U256::from_uint(300_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    let modulo = U256::halfmax();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(150_u8);
    let modulo = U256::from_uint(50_u8);
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 0
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    let modulo = U256::zero();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self != 0 and exp != 0 and modulo == 1
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    let modulo = U256::one();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    let modulo = U256::zero();
    a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // collectively
    for modulo in [U256::zero(), U256::one()]
    {
        for lhs in [U256::zero(), U256::from_uint(50_u8)]
        {
            for rhs in [U256::zero(), U256::from_uint(50_u8)]
            {
                let mut lhs = lhs.clone();
                println!("Originally, lhs = {}", lhs);
                assert_eq!(lhs.is_overflow(), false);
                assert_eq!(lhs.is_underflow(), false);
                assert_eq!(lhs.is_infinity(), false);
                assert_eq!(lhs.is_undefined(), false);
                assert_eq!(lhs.is_divided_by_zero(), false);
                assert_eq!(lhs.is_left_carry(), false);
                assert_eq!(lhs.is_right_carry(), false);

                lhs.panic_free_modular_pow_assign(&rhs, &modulo);
                println!("After lhs.panic_free_modular_pow_assign({}, {}), lhs = {}", rhs, modulo, lhs);
                assert_eq!(lhs.to_string(), "0");
                assert_eq!(lhs.is_overflow(), false);
                assert_eq!(lhs.is_underflow(), false);
                assert_eq!(lhs.is_infinity(), false);
                assert_eq!(lhs.is_undefined(), true);
                assert_eq!(lhs.is_divided_by_zero(), false);
                assert_eq!(lhs.is_left_carry(), false);
                assert_eq!(lhs.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_iroot()
{
    println!("biguint_panic_free_iroot");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(8_u8);
    let res = a_biguint.panic_free_iroot(&exp);
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
    let res = a_biguint.panic_free_iroot(&exp);
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
    let res = a_biguint.panic_free_iroot(&exp);
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
    let res = a_biguint.panic_free_iroot(&exp);
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
    let res = a_biguint.panic_free_iroot(&exp);
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
    let res = a_biguint.panic_free_iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::zero();
    let res = a_biguint.panic_free_iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let exp = U256::zero();
    let res = a_biguint.panic_free_iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let exp = U256::zero();
    let res = a_biguint.panic_free_iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_iroot_assign()
{
    println!("biguint_panic_free_iroot_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    let exp = U256::zero();
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::one();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    a_biguint.panic_free_iroot_assign(&exp);
    println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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

fn biguint_panic_free_ilog()
{
    println!("biguint_panic_free_ilog");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    let res = a_biguint.panic_free_ilog(&base);
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
    let res = a_biguint.panic_free_ilog(&base);
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
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::zero();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::one();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res, U256::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), true);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = U256::from_uint(6_u8);
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = U256::zero();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let base = U256::one();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = U256::zero();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = U256::one();
    let res = a_biguint.panic_free_ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_ilog_assign()
{
    println!("biguint_panic_free_ilog_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

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
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    let base = U256::zero();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = U256::one();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = U256::from_uint(6_u8);
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = U256::zero();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = U256::one();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = U256::zero();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
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

    let base = U256::one();
    a_biguint.panic_free_ilog_assign(&base);
    println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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

fn biguint_panic_free_ilog2()
{
    println!("biguint_panic_free_ilog2");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    let a_biguint = U256::from_uint(64_u8);
    let res = a_biguint.panic_free_ilog2();
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
    let res = a_biguint.panic_free_ilog2();
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
    let res = a_biguint.panic_free_ilog2();
    println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.panic_free_ilog2();
    println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_ilog2_assign()
{
    println!("biguint_panic_free_ilog2_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_uint(64_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_ilog2_assign();
    println!("After a_biguint.panic_free_ilog2_assign(),\na_biguint = {}.", a_biguint);
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

    a_biguint.panic_free_ilog2_assign();
    println!("After a_biguint.panic_free_ilog2_assign(),\na_biguint = {}.", a_biguint);
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

    a_biguint.panic_free_ilog2_assign();
    println!("After a_biguint.panic_free_ilog2_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    a_biguint.panic_free_ilog2_assign();
    println!("After a_biguint.panic_free_ilog2_assign(),\na_biguint = {}.", a_biguint);
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

fn biguint_panic_free_ilog10()
{
    println!("biguint_panic_free_ilog10");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    let a_biguint = U256::from_uint(10000_u32);
    let res = a_biguint.panic_free_ilog10();
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
    let res = a_biguint.panic_free_ilog10();
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
    let res = a_biguint.panic_free_ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.panic_free_ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_ilog10_assign()
{
    println!("biguint_panic_free_ilog10_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_uint(10000_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_ilog10_assign();
    println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
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

    a_biguint.panic_free_ilog10_assign();
    println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
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

    a_biguint.panic_free_ilog10_assign();
    println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
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

    a_biguint.panic_free_ilog10_assign();
    println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
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

fn biguint_panic_free_next_multiple()
{
    biguint_panic_free_next_multiple_of_uint();
    biguint_panic_free_next_multiple_of_assign_uint();
    biguint_panic_free_modular_next_multiple_of_uint();
    biguint_panic_free_modular_next_multiple_of_assign_uint();
    biguint_panic_free_next_multiple_of();
    biguint_panic_free_next_multiple_of_assign();
    biguint_panic_free_modular_next_multiple_of();
    biguint_panic_free_modular_next_multiple_of_assign();
    
}

fn biguint_panic_free_next_multiple_of_uint()
{
    println!("biguint_panic_free_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_modular_next_multiple_of_uint()
{
    println!("biguint_modular_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
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
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
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

fn biguint_panic_free_modular_next_multiple_of_assign_uint()
{
    println!("biguint_panic_free_modular_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
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

fn biguint_panic_free_next_multiple_of()
{
    println!("biguint_panic_free_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = U256::from(586478_u32);
    let multiple = a_biguint.panic_free_next_multiple_of(&num);
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
    let multiple = a_biguint.panic_free_next_multiple_of(&num);
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
    let num = U256::zero();
    let multiple = a_biguint.panic_free_next_multiple_of(&num);
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

fn biguint_panic_free_next_multiple_of_assign()
{
    println!("biguint_panic_free_next_multiple_of_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let num = UU32::from(586478_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign(&num);
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
    let num = UU32::from(586478_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign(&num);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "448670");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let num = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign(&num);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

fn biguint_panic_free_modular_next_multiple_of()
{
    println!("biguint_panic_free_modular_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u16);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = U256::from(100_u8);
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::zero();
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::from(200_u8);
    let modulo = U256::from(100_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::from(100_u8);
    let modulo = U256::zero();
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::from(100_u8);
    let modulo = U256::one();
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::zero();
    let modulo = U256::zero();
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    let num = U256::zero();
    let modulo = U256::one();
    let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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

fn biguint_panic_free_modular_next_multiple_of_assign()
{
    println!("biguint_panic_free_modular_next_multiple_of_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u32);

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
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
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
    let mut a_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
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
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
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
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = U256::zero();
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

    let num = U256::from(200_u8);
    let modulo = U256::from(100_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

    let num = U256::from(100_u8);
    let modulo = U256::zero();
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

    let num = U256::from(100_u8);
    let modulo = U256::one();
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

    let num = U256::zero();
    let modulo = U256::zero();
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

    let num = U256::zero();
    let modulo = U256::one();
    a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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

fn biguint_panic_free_misc()
{
    biguint_panic_free_gcd_uint();
    biguint_panic_free_gcd_assign_uint();
    biguint_panic_free_lcm_uint();
    biguint_panic_free_lcm_assign_uint();
    biguint_panic_free_gcd();
    biguint_panic_free_gcd_assign();
    biguint_panic_free_lcm();
    biguint_panic_free_lcm_assign();
}

fn biguint_panic_free_gcd_uint()
{
    println!("biguint_panic_free_gcd_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // normal case
    let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "11111");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Two prime numbers
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = 176599892424056297732340280216263039863_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    let b_biguint = 256529360383586277064974026736439112491_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Same numbers
    let a_biguint = U256::from_string("71263413766404235019454912736237592261").unwrap();
    let b_biguint = 71263413766404235019454912736237592261_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "71263413766404235019454912736237592261");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let a_biguint = U256::from_string("311334932976109408876358199994339131327").unwrap();
    // let mut a_biguint = U256::from_string("103778310992036469625452733331446377109").unwrap().wrapping_mul_uint(3_u8);
    let b_biguint = 103778310992036469625452733331446377109_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "103778310992036469625452733331446377109");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let b_biguint = 25029766050440185546875_u128;
    // let b_biguint = 3_u128.pow(25_u32).wrapping_mul(5_u128.pow(12_u32)).wrapping_mul(11_u128.pow(2_u32));
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and non-zero
    let a_biguint = U256::zero();
    let b_biguint = 103778310992036469625452733331446377109_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // non-zero and zero
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let b_biguint = 0_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and zero
    let a_biguint = U256::zero();
    let b_biguint = 0_u128;
    let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_gcd_assign_uint()
{
    println!("biguint_panic_free_gcd_assign_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // normal case
    let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 77777666665555544444333332222211111_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "11111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Two prime numbers
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 176599892424056297732340280216263039863_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 77777666665555544444333332222211111_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let mut a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 256529360383586277064974026736439112491_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Same numbers
    let mut a_biguint = U256::from_string("71263413766404235019454912736237592261").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 71263413766404235019454912736237592261_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "71263413766404235019454912736237592261");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let mut a_biguint = U256::from_string("311334932976109408876358199994339131327").unwrap();
    // let mut a_biguint = U256::from_string("103778310992036469625452733331446377109").unwrap().wrapping_mul_uint(3_u8);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 103778310992036469625452733331446377109_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "103778310992036469625452733331446377109");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 25029766050440185546875_u128;
    // let b_biguint = 3_u128.pow(25_u32).wrapping_mul(5_u128.pow(12_u32)).wrapping_mul(11_u128.pow(2_u32));
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and non-zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 103778310992036469625452733331446377109_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // non-zero and zero
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 0_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 0_u128;
    a_biguint.panic_free_gcd_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
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

fn biguint_panic_free_lcm_uint()
{
    println!("biguint_panic_free_lcm_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    // normal case
    let a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "7777922224222246666944447444475555866662777741110777774888865555388888");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Two prime numbers
    let a_biguint = U512::from_string("6803131165750672834156364579962694397471399207621174936018049247058097685071").unwrap();
    let b_biguint = 176599892424056297732340280216263039863_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1201432232018313536575078427518720257429815777213343847736733246472480617592688699762764735843270475023457692985273");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let a_biguint = U512::from_string("44252664306827291403239758473867025040196893255067151905832712870552757072629").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "3441868973519140676288607887594334453559862957523356796877044853256166361556295667060287344153336903049997780819");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    let b_biguint = 256529360383586277064974026736439112491_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "199522550818427434557973689651667058038144567865901188449215831677613012159957775002");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Same numbers
    let a_biguint = U256::from_string("71263413766404235019454912736237592261").unwrap();
    let b_biguint = 71263413766404235019454912736237592261_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "71263413766404235019454912736237592261");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let a_biguint = U256::from_string("311334932976109408876358199994339131327").unwrap();
    // let mut a_biguint = U512::from_string("103778310992036469625452733331446377109").unwrap().wrapping_mul_uint(3_u8);
    let b_biguint = 103778310992036469625452733331446377109_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "311334932976109408876358199994339131327");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let a_biguint = U512::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    let b_biguint = 25029766050440185546875_u128;
    // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "20596479741978911975639783055646618200359178304364816695371910650463951431749917999104000000000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and non-zero
    let a_biguint = U256::zero();
    let b_biguint = 103778310992036469625452733331446377109_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // non-zero and zero
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let b_biguint = 0_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and zero
    let a_biguint = U256::zero();
    let b_biguint = 0_u128;
    let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_lcm_assign_uint()
{
    println!("biguint_panic_free_lcm_assign_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // normal case
    let mut a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 77777666665555544444333332222211111_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "7777922224222246666944447444475555866662777741110777774888865555388888");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Two prime numbers
    let mut a_biguint = U512::from_string("6803131165750672834156364579962694397471399207621174936018049247058097685071").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 176599892424056297732340280216263039863_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1201432232018313536575078427518720257429815777213343847736733246472480617592688699762764735843270475023457692985273");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let mut a_biguint = U512::from_string("44252664306827291403239758473867025040196893255067151905832712870552757072629").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 77777666665555544444333332222211111_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "3441868973519140676288607887594334453559862957523356796877044853256166361556295667060287344153336903049997780819");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let mut a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 256529360383586277064974026736439112491_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "199522550818427434557973689651667058038144567865901188449215831677613012159957775002");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Same numbers
    let mut a_biguint = U256::from_string("71263413766404235019454912736237592261").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 71263413766404235019454912736237592261_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "71263413766404235019454912736237592261");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let mut a_biguint = U256::from_string("311334932976109408876358199994339131327").unwrap();
    // let mut a_biguint = U512::from_string("103778310992036469625452733331446377109").unwrap().wrapping_mul_uint(3_u8);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 103778310992036469625452733331446377109_u128;    assert_eq!(a_biguint.is_overflow(), false);
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "311334932976109408876358199994339131327");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let mut a_biguint = U512::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 25029766050440185546875_u128;
    // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "20596479741978911975639783055646618200359178304364816695371910650463951431749917999104000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and non-zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 103778310992036469625452733331446377109_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // non-zero and zero
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 0_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 0_u128;
    a_biguint.panic_free_lcm_assign_uint(b_biguint);
    println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
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

fn biguint_panic_free_gcd()
{
    println!("biguint_panic_free_gcd()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // normal case
    let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "27");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Two prime numbers
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Same numbers
    let a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and non-zero
    let a_biguint = U256::zero();
    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // non-zero and zero
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and zero
    let a_biguint = U256::zero();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_gcd_assign()
{
    println!("biguint_panic_free_gcd_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u8);

    // normal case
    let mut a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "27");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Two prime numbers
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let mut a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Same numbers
    let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and non-zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // non-zero and zero
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint.panic_free_gcd_assign(&b_biguint);
    println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
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

fn biguint_panic_free_lcm()
{
    println!("biguint_panic_free_lcm()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u64);

    // normal case
    let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Two prime numbers
    let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Same numbers
    let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and non-zero
    let a_biguint = U256::zero();
    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // non-zero and zero
    let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // zero and zero
    let a_biguint = U256::zero();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), true);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_lcm_assign()
{
    println!("biguint_panic_free_lcm_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Panic_Free;
    define_utypes_with!(u128);

    // normal case
    let mut a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Two prime numbers
    let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a prime number and other is a composite number
    let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self is a composite number and another is prime number
    let mut a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Same numbers
    let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // one prime number and its multiple
    let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // two relatively prime numbers
    let mut a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and non-zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // non-zero and zero
    let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // zero and zero
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint.panic_free_lcm_assign(&b_biguint);
    println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
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
