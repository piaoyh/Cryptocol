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
    biguint_modular_add_uint();
    biguint_modular_add_assign_uint();
    biguint_modular_sub_uint();
    biguint_modular_sub_assign_uint();
    biguint_modular_mul_uint();
    biguint_modular_mul_assign_uint();
    biguint_modular_div_uint();
    biguint_modular_div_assign_uint();
    biguint_modular_rem_uint();
    biguint_modular_rem_assign_uint();

    biguint_modular_next_multiple_of_uint();
    biguint_modular_next_multiple_of_assign_uint();
    biguint_modular_pow_uint();
    biguint_modular_pow_assign_uint();

    biguint_modular_add();
    biguint_modular_add_assign();
    biguint_modular_sub();
    biguint_modular_sub_assign();
    biguint_modular_mul();
    biguint_modular_mul_assign();
    biguint_modular_div();
    biguint_modular_div_assign();
    biguint_modular_rem();
    biguint_modular_rem_assign();
    
    biguint_modular_next_multiple_of();
    biguint_modular_next_multiple_of_assign();
    biguint_modular_pow();
    biguint_modular_pow_assign();
}

fn biguint_modular_add_uint()
{
    println!("biguint_modular_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_sub_uint()
{
    println!("biguint_modular_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_mul_uint()
{
    println!("biguint_modular_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_div_uint()
{
    println!("biguint_modular_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_rem_uint()
{
    println!("biguint_modular_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_next_multiple_of_uint()
{
    println!("biguint_modular_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_pow_uint()
{
    println!("biguint_modular_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
    define_utypes_with!(u32);

    // Noraml case 1
    let a_biguint = U256::from_uint(10_u8);
    let exp = 30_u8;
    let modulo = U256::halfmax();
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
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
    let exp = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
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
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let a_biguint = UU32::from_uint(300_u16);
    let exp = 30_u8;
    let modulo = U256::from_uint(100_u8);
    let res = a_biguint.modular_pow_uint(exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let _a_biguint = UU32::zero();
    let _exp = 0_u8;
    let _modulo = U256::halfmax();
    // It will panic.
    // let res = _a_biguint.modular_pow_uint(_exp, &_modulo);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let _a_biguint = UU32::zero();
    let _exp = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow_uint(_exp, &_modulo);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let _a_biguint = UU32::from_uint(300_u16);
    let _exp = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow_uint(_exp, &_modulo);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let _a_biguint = UU32::from_uint(300_u16);
    let _exp = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow_uint(_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 0
    let _a_biguint = U256::from_uint(10_u8);
    let _exp = 100_u8;
    let _modulo = U256::zero();
    // It will panic!
    // let _res = a_biguint.modular_pow_uint(_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 1
    let _a_biguint = U256::from_uint(10_u8);
    let _exp = 100_u8;
    let _modulo = U256::one();
    // It will panic!
    // let _res = a_biguint.modular_pow_uint(_exp, &_modulo);

    // self == 0 and exp == 0 and modulo == 0
    let _a_biguint = U256::zero();
    let _exp = 0_u8;
    let _modulo = U256::zero();
    // It will panic!
    // let _res = a_biguint.modular_pow_uint(_exp, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_pow_assign_uint()
{
    println!("biguint_modular_pow_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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

    let exp = 30_u8;
    let modulo = U256::halfmax();
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign_uint(exp, &modulo);
    println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    let _modulo = U256::halfmax();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let mut _a_biguint = U256::zero();
    let _exp = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let mut _a_biguint = U256::from_uint(300_u16);
    let _exp = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let mut _a_biguint = U256::from_uint(300_u16);
    let _exp = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 0
    let mut _a_biguint = U256::from_uint(10_u8);
    let _exp = 100_u8;
    let _modulo = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 1
    let _a_biguint = U256::from_uint(10_u8);
    let _exp = 100_u8;
    let _modulo = U256::one();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);

    // self == 0 and exp == 0 and modulo == 0
    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    let _modulo = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_add()
{
    println!("biguint_modular_add");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_sub()
{
    println!("biguint_modular_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_mul()
{
    println!("biguint_modular_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_div()
{
    println!("biguint_modular_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_rem()
{
    println!("biguint_modular_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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

fn biguint_modular_next_multiple_of()
{
    println!("biguint_modular_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
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
    use cryptocol::number::BigUInt_Modular;
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
fn biguint_modular_pow()
{
    println!("biguint_modular_pow()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
    define_utypes_with!(u8);

    // Noraml case 1
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let modulo = UU32::halfmax();
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    let modulo = U256::halfmax();
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
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
    let exp = UU32::from_uint(200_u8);
    let modulo = UU32::from_uint(100_u8);
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
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
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == multiple of modulo and exp != 0 and modulo != 0
    let a_biguint = UU32::from_uint(300_u16);
    let exp = UU32::from_uint(30_u8);
    let modulo = U256::from_uint(100_u8);
    let res = a_biguint.modular_pow(&exp, &modulo);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let _a_biguint = UU32::zero();
    let _exp = UU32::zero();
    let _modulo = UU32::halfmax();
    // It will panic.
    // let res = _a_biguint.modular_pow(_exp, &_modulo);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let _a_biguint = UU32::zero();
    let _exp = UU32::from_uint(200_u8);
    let _modulo = UU32::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow(&_exp, &_modulo);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let _a_biguint = UU32::from_uint(300_u16);
    let _exp = UU32::zero();
    let _modulo = UU32::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow(&_exp, &_modulo);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let _a_biguint = UU32::from_uint(300_u16);
    let _exp = UU32::from_uint(200_u8);
    let _modulo = UU32::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.modular_pow(&_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 0
    let _a_biguint = UU32::from_uint(10_u8);
    let _exp = UU32::from_uint(100_u8);
    let _modulo = UU32::zero();
    // It will panic!
    // let _res = a_biguint.modular_pow(&_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 1
    let _a_biguint = UU32::from_uint(10_u8);
    let _exp = UU32::from_uint(100_u8);
    let _modulo = UU32::one();
    // It will panic!
    // let _res = a_biguint.modular_pow(&_exp, &_modulo);

    // self == 0 and exp == 0 and modulo == 0
    let _a_biguint = UU32::zero();
    let _exp = UU32::zero();
    let _modulo = UU32::zero();
    // It will panic!
    // let _res = a_biguint.modular_pow(&_exp, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_pow_assign()
{
    println!("biguint_modular_pow_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Modular;
    define_utypes_with!(u16);

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
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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

    let exp = U256::from_uint(30_u8);
    let modulo = U256::halfmax();
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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

    let exp = U256::from_uint(30_u8);
    let modulo = U256::from_uint(100_u8);
    a_biguint.modular_pow_assign(&exp, &modulo);
    println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // self == 0 and exp == 0 and modulo != 0
    let mut _a_biguint = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::zero();
    let _modulo = U256::halfmax();
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self == 0 and exp == multiple of modulo and modulo != 0
    let mut _a_biguint = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self == multiple of modulo and exp == 0 and modulo != 0
    let mut _a_biguint = U256::from_uint(300_u16);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    let mut _a_biguint = U256::from_uint(300_u16);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 0
    let mut _a_biguint = U256::from_uint(10_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::from_uint(100_u8);
    let _modulo = U256::zero();
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self != 0 and exp != 0 and modulo == 1
    let _a_biguint = U256::from_uint(10_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    let _exp = U256::from_uint(100_u8);
    let _modulo = U256::one();
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);

    // self == 0 and exp == 0 and modulo == 0
    let mut _a_biguint = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _exp = U256::zero();
    let _modulo = U256::zero();
    // It will panic!
    // _a_biguint.modular_pow_assign(&_exp, &_modulo);
    println!("---------------------------");
}
