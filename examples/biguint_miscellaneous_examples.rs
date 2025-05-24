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
    biguint_operators_arithmatic_main();
    biguint_operators_bit_operation_main();
    biguint_operators_miscellaneous_main();
    biguint_implementation_miscellaneous_main();
}


fn biguint_operators_arithmatic_main()
{
    biguint_op_add();
    biguint_op_add_uint();
    biguint_op_add_assign();
    biguint_op_add_assign_uint();
    biguint_op_sub();
    biguint_op_sub_uint();
    biguint_op_sub_assign();
    biguint_op_sub_assign_uint();
    biguint_op_mul();
    biguint_op_mul_uint();
    biguint_op_mul_assign();
    biguint_op_mul_assign_uint();
    biguint_op_div();
    biguint_op_div_uint();
    biguint_op_div_assign();
    biguint_op_div_assign_uint();
    biguint_op_rem();
    biguint_op_rem_uint();
    biguint_op_rem_assign();
    biguint_op_rem_assign_uint();
}

fn biguint_op_add()
{
    println!("biguint_op_add()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    let a_biguint = U512::max() - 1_u8;
    let one_biguint = U512::one();
    let res = a_biguint.clone() + one_biguint.clone();
    println!("{} + {} = {}", a_biguint, one_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    let a_biguint = U512::max() - 1_u8;
    let two_biguint = U512::from_uint(2_u8);
    let res = a_biguint.clone() + two_biguint.clone();
    println!("{} + {} = {}", a_biguint, two_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    let a_biguint = U512::max() - 1_u8;
    let three_biguint = U512::from_uint(3_u8);
    let res = a_biguint.clone() + three_biguint.clone();
    println!("{} + {} = {}", a_biguint, three_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max() - 1_u8;
    let one_biguint = U512::one();
    let _res = a_biguint + one_biguint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, one_biguint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint and one_biguint.

    let a_biguint = U512::max() - 1_u8;
    let two_biguint = U512::from_uint(2_u8);
    let _res = a_biguint + two_biguint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, two_biguint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint and two_biguint.

    let a_biguint = U512::max() - 1_u8;
    let three_biguint = U512::from_uint(3_u8);
    let _res = a_biguint + three_biguint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, three_biguint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint and three_biguint.
    println!("---------------------------");
}

fn biguint_op_add_uint()
{
    println!("biguint_op_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U512::max() - 1_u16;
    let one_uint = 1_u16;
    let res = a_biguint.clone() + one_uint;
    println!("{} + {} = {}", a_biguint, one_uint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max() - 1_u16;
    let two_uint = 2_u16;
    let res = a_biguint.clone() + two_uint;
    println!("{} + {} = {}", a_biguint, two_uint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max() - 1_u16;
    let three_uint = 3_u16;
    let res = a_biguint.clone() + three_uint;
    println!("{} + {} = {}", a_biguint, three_uint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max() - 1_u16;
    let one_uint = 1_u16;
    let _res = a_biguint + one_uint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, one_uint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint.

    let a_biguint = U512::max() - 1_u16;
    let two_uint = 2_u16;
    let _res = a_biguint + two_uint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, two_uint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint.

    let a_biguint = U512::max() - 1_u16;
    let three_uint = 3_u16;
    let _res = a_biguint + three_uint;
    // It cannot be compiled!
    // println!("{} + {} = {}", a_biguint, three_uint, _res);
    // The operator '+' swallowed (took the ownership of) a_biguint.
    println!("---------------------------");
}

fn biguint_op_add_assign()
{
    println!("biguint_op_add_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    
    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint += one_biguint.clone();
    println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
    assert_eq!(a_biguint, U512::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let two_biguint = U512::from_uint(2_u8);
    a_biguint += two_biguint.clone();
    println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let three_biguint = U512::from_uint(3_u8);
    a_biguint += three_biguint.clone();
    println!("After a_biguint += {},\ta_biguint = {}", three_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    let one_biguint = U512::one();
    a_biguint += one_biguint;
    // It cannot be compiled!
    // println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
    // The operator '+=' swallowed (took the ownership of) one_biguint.

    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    let two_biguint = U512::from_uint(2_u8);
    a_biguint += two_biguint.clone();
    // It cannot be compiled!
    // println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    // The operator '+=' swallowed (took the ownership of) two_biguint.

    let mut a_biguint = U512::max() - 1_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    let three_biguint = U512::from_uint(3_u8);
    a_biguint += three_biguint.clone();
    // It cannot be compiled!
    // println!("After a_biguint += {}, a_biguint = {}", three_biguint, a_biguint);
    // The operator '+=' swallowed (took the ownership of) three_biguint.
    println!("---------------------------");
}

fn biguint_op_add_assign_uint()
{
    println!("biguint_op_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = UU64::max() - 1_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_uint = 1_u64;
    a_biguint += one_uint;
    println!("After a_biguint += {}, a_biguint = {}", one_uint, a_biguint);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::max() - 1_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let two_uint = 2_u64;
    a_biguint += two_uint;
    println!("After a_biguint += {}, a_biguint = {}", two_uint, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max() - 1_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let three_uint = 3_u64;
    a_biguint += three_uint;
    println!("After a_biguint += {}, a_biguint = {}", three_uint, a_biguint);
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

fn biguint_op_sub()
{
    println!("biguint_op_sub");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::one();
    let one_biguint = U512::one();
    let res = a_biguint.clone() - one_biguint.clone();
    println!("{} - {} = {}", a_biguint, one_biguint, res);
    assert_eq!(res, U512::zero());
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let two_biguint = U512::from_uint(2_u8);
    let res = a_biguint.clone() - two_biguint.clone();
    println!("{} - {} = {}", a_biguint, two_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let three_biguint = U512::from_uint(3_u8);
    let res = a_biguint.clone() - three_biguint.clone();
    println!("{} - {} = {}", a_biguint, three_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let one_biguint = U512::one();
    let _res = a_biguint - one_biguint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_biguint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint and one_biguint.

    let a_biguint = U512::one();
    let two_biguint = U512::from_uint(2_u8);
    let _res = a_biguint - two_biguint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_biguint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint and two_biguint.

    let a_biguint = U512::one();
    let three_biguint = U512::from_uint(3_u8);
    let _res = a_biguint - three_biguint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_biguint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint and three_biguint.
    println!("---------------------------");
}

fn biguint_op_sub_uint()
{
    println!("biguint_op_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U512::one();
    let one_uint = 1_u8;
    let res = a_biguint.clone() - one_uint.clone();
    println!("{} - {} = {}", a_biguint, one_uint, res);
    assert_eq!(res, U512::zero());
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let two_uint = 2_u8;
    let res = a_biguint.clone() - two_uint.clone();
    println!("{} - {} = {}", a_biguint, two_uint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let three_uint = 3_u8;
    let res = a_biguint.clone() - three_uint.clone();
    println!("{} - {} = {}", a_biguint, three_uint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let one_uint = 1_8;
    let _res = a_biguint - one_uint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_uint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint.

    let a_biguint = U512::one();
    let two_uint = 2_u8;
    let _res = a_biguint - two_uint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_uint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint.

    let a_biguint = U512::one();
    let three_uint = 3_u8;
    let _res = a_biguint - three_uint;
    // It cannot be compiled!
    // println!("{} - {} = {}", a_biguint, one_uint, _res);
    // The operator '-' swallowed (took the ownership of) a_biguint.
    println!("---------------------------");
}

fn biguint_op_sub_assign()
{
    println!("biguint_op_sub_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint -= one_biguint.clone();
    println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let two_biguint = U512::from_uint(2_u8);
    a_biguint -= two_biguint.clone();
    println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let three_biguint = U512::from_uint(3_u8);
    a_biguint -= three_biguint.clone();
    println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    let one_biguint = U512::one();
    a_biguint -= one_biguint;
    // It cannot be compiled!
    // println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
    // The operator '-=' swallowed (took the ownership of) one_biguint.

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    let two_biguint = U512::from_uint(2_u8);
    a_biguint -= two_biguint.clone();
    // It cannot be compiled!
    // println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
    // The operator '-=' swallowed (took the ownership of) two_biguint.

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    let three_biguint = U512::from_uint(3_u8);
    a_biguint -= three_biguint.clone();
    // It cannot be compiled!
    // println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    // The operator '-=' swallowed (took the ownership of) three_biguint.
    println!("---------------------------");
}

fn biguint_op_sub_assign_uint()
{
    println!("biguint_op_sub_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_uint = 1_u32;
    a_biguint -= one_uint;
    println!("After a_biguint -= {}, a_biguint = {}", one_uint, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let two_uint = 2_u32;
    a_biguint -= two_uint;
    println!("After a_biguint -= {}, a_biguint = {}", two_uint, a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let three_uint = 3_u32;
    a_biguint -= three_uint;
    println!("After a_biguint -= {}, a_biguint = {}", three_uint, a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_op_mul()
{
    println!("biguint_op_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.clone() * b_biguint.clone();
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
    let res = a_biguint.clone() * b_biguint.clone();
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let _res = a_biguint * b_biguint;
    // It cannot be compiled!
    // println!("{} X {} = {}", a_biguint, b_biguint, res);
    // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    
    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let _res = a_biguint * b_biguint;
    // It cannot be compiled!
    // println!("{} X {} = {}", a_biguint, b_biguint, res);
    // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    println!("---------------------------");
}

fn biguint_op_mul_uint()
{
    println!("biguint_op_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_uint = 248_u128;
    let res = a_biguint.clone() * b_uint;
    println!("{} X {} = {}", a_biguint, b_uint, res);
    assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u128;
    let res = a_biguint.clone() * b_uint;
    println!("{} X {} = {}", a_biguint, b_uint, res);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_uint = 248_u128;
    let _res = a_biguint * b_uint;
    // It cannot be compiled!
    // println!("{} X {} = {}", a_biguint, b_uint, _res);
    // The operator '*' swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u128;
    let _res = a_biguint * b_uint;
    // It cannot be compiled!
    // println!("{} X {} = {}", b_biguint, b_uint, _res);
    // The operator '*' swallowed (took the ownership of) a_biguint.
    println!("---------------------------");
}

fn biguint_op_mul_assign()
{
    println!("biguint_op_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

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
    a_biguint *= b_biguint.clone();
    println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
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
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_uint(248_u8);
    a_biguint *= b_biguint.clone();
    println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    let b_biguint = U256::from_uint(248_u8);
    a_biguint *= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    
    let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    let b_biguint = U256::from_uint(248_u8);
    a_biguint *= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    println!("---------------------------");
}

fn biguint_op_mul_assign_uint()
{
    println!("biguint_op_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

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
    a_biguint *= b_uint;
    println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
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

    let b_uint = 248_u16;
    a_biguint *= b_uint;
    println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
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

fn biguint_op_div()
{
    println!("biguint_op_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.clone() / divisor.clone();
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
    let quotient = dividend.clone() / divisor.clone();
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
    // let quotient = _dividend.clone() / _divisor.clone();

    let _dividend = U256::zero();
    let _divisor = U256::zero();
    // It will panic!
    // let quotient = _dividend.clone() / _divisor.clone();

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let _quotient = dividend / divisor;
    // It cannot be compiled!
    // println!("{} / {} = {}", dividend, divisor, _quotient);
    // The operator '/' swallowed (took the ownership of) dividend and divisor.

    let dividend = U256::zero();
    let divisor = U256::from_uint(87_u8);
    let _quotient = dividend / divisor;
    // It cannot be compiled!
    // println!("{} / {} = {}", dividend, divisor, _quotient);
    // The operator '/' swallowed (took the ownership of) dividend and divisor.
    println!("---------------------------");
}

fn biguint_op_div_uint()
{
    println!("biguint_op_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u64;
    let quotient = dividend.clone() / divisor;
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
    let divisor = 87_u64;
    let quotient = dividend.clone() / divisor;
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
    let _divisor = 0_u64;
    // It will panic!
    // let quotient = _dividend.clone() / _divisor;

    let _dividend = U256::zero();
    let _divisor = 0_u64;
    // It will panic!
    // let quotient = _dividend.clone() / _divisor;

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u64;
    let _quotient = dividend / divisor;
    // It cannot be compiled!
    // println!("{} / {} = {}", dividend, divisor, _quotient);
    // The operator '/' swallowed (took the ownership of) dividend.

    let dividend = U256::zero();
    let divisor = 87_u64;
    let _quotient = dividend / divisor;
    // It cannot be compiled!
    // println!("{} / {} = {}", dividend, divisor, _quotient);
    // The operator '/' swallowed (took the ownership of) dividend.
    println!("---------------------------");
}

fn biguint_op_div_assign()
{
    println!("biguint_op_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

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
    a_biguint /= divisor.clone();
    println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
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
    a_biguint /= divisor.clone();
    println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = UU32::zero();
    // It will panic!
    // _a_biguint /= _divisor.clone();

    let mut _a_biguint = UU32::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = UU32::zero();
    // It will panic!
    // _a_biguint /= _divisor.clone();

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    let divisor = UU32::from_uint(87_u8);
    a_biguint /= divisor;
    // It cannot be compiled!
    // println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    // The operator '/=' swallowed (took the ownership of) divisor.

    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    let divisor = UU32::from_uint(87_u8);
    a_biguint /= divisor;
    // It cannot be compiled!
    // println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    // The operator '/=' swallowed (took the ownership of) divisor.
    println!("---------------------------");
}

fn biguint_op_div_assign_uint()
{
    println!("biguint_op_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

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
    a_biguint /= divisor;
    println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
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
    a_biguint /= divisor;
    println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.wrapping_div_assign_uint(_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = 0_u8;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.wrapping_div_assign_uint(_divisor);
    println!("---------------------------");
}

fn biguint_op_rem()
{
    println!("biguint_op_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.clone() % divisor.clone();
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
    let remainder = dividend.clone() % divisor.clone();
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

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let _remainder = dividend % divisor;
    // It cannot be compiled!
    // println!("{} % {} = {}", dividend, divisor, _remainder);
    // The operator '%' swallowed (took the ownership of) dividend and divisor.

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let _remainder = dividend % divisor;
    // It cannot be compiled!
    // println!("{} % {} = {}", dividend, divisor, _remainder);
    // The operator '%' swallowed (took the ownership of) dividend and divisor.
    println!("---------------------------");
}

fn biguint_op_rem_uint()
{
    println!("biguint_op_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u32;
    let remainder = dividend.clone() % divisor;
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    let dividend = UU32::zero();
    let divisor = 87_u32;
    let remainder = dividend.clone() % divisor;
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u32;
    // It will panic!
    // let remainder = _dividend.clone() % _divisor;

    let _dividend = UU32::zero();
    let _divisor = 0_u32;
    // It will panic!
    // let remainder = _dividend.clone() % _divisor;

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u32;
    let _remainder = dividend % divisor;
    // It cannot be compiled!
    // println!("{} % {} = {}", dividend, divisor, _remainder);
    // The operator '%' swallowed (took the ownership of) dividend.

    let dividend = UU32::zero();
    let divisor = 87_u32;
    let _remainder = dividend % divisor;
    // It cannot be compiled!
    // println!("{} % {} = {}", dividend, divisor, _remainder);
    // The operator '%' swallowed (took the ownership of) dividend.
    println!("---------------------------");
}

fn biguint_op_rem_assign()
{
    println!("biguint_op_rem_assign");
    use std::str::FromStr;
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
    a_biguint %= divisor.clone();
    println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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
    a_biguint %= divisor.clone();
    println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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
    // _a_biguint %= _divisor.clone();

    let mut _a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    // It will panic!
    // _a_biguint %= _divisor.clone();

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    let divisor = U256::from_uint(87_u8);
    a_biguint %= divisor;
    // It cannot be compiled!
    // println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    // The operator %= swallowed (took the ownership of) divisor.
    
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    let divisor = U256::from_uint(87_u8);
    a_biguint %= divisor;
    // It cannot be compiled!
    // println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    // The operator %= swallowed (took the ownership of) divisor.
    println!("---------------------------");
}

fn biguint_op_rem_assign_uint()
{
    println!("biguint_op_rem_assign_uint");
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

    let divisor = 87_u128;
    a_biguint %= divisor;
    println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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

    let divisor = 87_u128;
    a_biguint %= divisor;
    println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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
    let _divisor = 0_u128;
    // It will panic!
    // _a_biguint %= _divisor;

    let mut _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _divisor = 0_u128;
    // It will panic!
    // _a_biguint %= _divisor;
    println!("---------------------------");
}


fn biguint_operators_bit_operation_main()
{
    biguint_operator_shift_left();
    biguint_operator_shift_left_assign();
    biguint_operator_shift_right();
    biguint_operator_shift_right_assign();
    biguint_operator_and();
    biguint_operator_and_assign();
    biguint_operator_or();
    biguint_operator_or_assign();
    biguint_operator_xor();
    biguint_operator_xor_assign();
    biguint_operator_flip();
}

fn biguint_operator_shift_left()
{
    println!("biguint_operator_shift_left");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.clone() << n;
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
    let n = 4_u16;
    let res = a_biguint.clone() << n;
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
    let n = 128_u32;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_u64;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_u128;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_usize;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_i8;
    let res = a_biguint.clone() << n;
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
    let n = 4_i16;
    let res = a_biguint.clone() << n;
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
    let n = 128_i32;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_i64;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_i128;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_isize;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -3_i8;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    let n = -4_i16;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -128_i32;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -256_i64;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -512_i128;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -1024_isize;
    let res = a_biguint.clone() << n;
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    #[cfg(test)]
    biguint_compile_fail_operator_shift_left();
    println!("---------------------------");
}

#[test]
fn biguint_compile_fail_operator_shift_left()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 4_u16;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 128_u32;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_u64;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_u128;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_usize;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_i8;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 4_i16;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 128_i32;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_i64;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_i128;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_isize;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -3_i8;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    let n = -4_i16;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -128_i32;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -256_i64;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -512_i128;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -1024_isize;
    let _res = a_biguint << n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator << swallowed (took the ownership of) a_biguint.
}

fn biguint_operator_shift_left_assign()
{
    println!("biguint_operator_shift_left_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 3_u8;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 4_u16;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 128_u32;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 256_u64;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 512_u128;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 1024_usize;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 3_i8;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011000_00000111_11111000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 4_i16;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 128_i32;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 256_i64;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 512_i128;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 1024_isize;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -3_i8;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -4_i16;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -128_i32;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -256_i64;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -512_i128;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -1024_isize;
    a_biguint <<= n;
    println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);
    println!("---------------------------");
}

fn biguint_operator_shift_right()
{
    println!("biguint_operator_shift_right");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.clone() >> n;
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
    let n = 4_u16;
    let res = a_biguint.clone() >> n;
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
    let n = 128_u32;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_u64;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_u128;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_usize;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 3_i8;
    let res = a_biguint.clone() >> n;
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
    let n = 4_i16;
    let res = a_biguint.clone() >> n;
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
    let n = 128_i32;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_i64;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_i128;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_isize;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -3_i8;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -4_i16;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -128_i32;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -256_i64;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -512_i128;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -1024_isize;
    let res = a_biguint.clone() >> n;
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)]
    biguint_compile_fail_operator_shift_right();
    println!("---------------------------");
}

#[test]
fn biguint_compile_fail_operator_shift_right()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 4_u16;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 128_u32;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_u64;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_u128;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 1024_usize;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -3_i8;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    let n = -4_i16;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = -128_i32;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -256_i64;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -512_i128;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = -1024_isize;
    let _res = a_biguint >> n;
    // It cannot be compiled!
    #[cfg(compile_fail)]    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator >> swallowed (took the ownership of) a_biguint.
}

fn biguint_operator_shift_right_assign()
{
    println!("biguint_operator_shift_right_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 3_u8;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 4_u16;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 128_u32;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 256_u64;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 512_u128;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 1024_usize;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);
    
    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 3_i8;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 4_i16;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 128_i32;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 256_i64;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 512_i128;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 1024_isize;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -3_i8;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -4_i16;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -128_i32;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -256_i64;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -512_i128;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = -1024_isize;
    a_biguint >>= n;
    println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_operator_and()
{
    println!("biguint_operator_and");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.clone() & b_biguint.clone();
    
    println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let c_biguint = a_biguint.clone() & b_biguint.clone();
    
    println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint, a_biguint);
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.clone() & b_biguint.clone();
    
    println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let _c_biguint = a_biguint & b_biguint;
    // It cannot be compiled!
    // println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator & swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let _c_biguint = a_biguint & b_biguint;
    // It cannot be compiled!
    // println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator & swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let _c_biguint = a_biguint & b_biguint;
    // It cannot be compiled!
    // println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    println!("---------------------------");
}

fn biguint_operator_and_assign()
{
    println!("biguint_operator_and_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint &= b_biguint.clone();
    println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::max();
    a_biguint &= b_biguint.clone();
    println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint &= b_biguint.clone();
    println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint &= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator &= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::max();
    a_biguint &= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator &= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::zero();
    a_biguint &= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator &= swallowed (took the ownership of) b_biguint.
    println!("---------------------------");
}

fn biguint_operator_or()
{
    println!("biguint_operator_or");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.clone() | b_biguint.clone();
    
    println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let c_biguint = a_biguint.clone() | b_biguint.clone();
    
    println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.clone() | b_biguint.clone();
    
    println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let _c_biguint = a_biguint | b_biguint;
    // It cannot be compiled!
    // println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator | swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let _c_biguint = a_biguint | b_biguint;
    // It cannot be compiled!
    // println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator | swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let _c_biguint = a_biguint | b_biguint;
    // It cannot be compiled!
    // println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    println!("---------------------------");
}

fn biguint_operator_or_assign()
{
    println!("biguint_operator_or_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint |= b_biguint.clone();
    println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::max();
    a_biguint |= b_biguint.clone();
    println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint |= b_biguint.clone();
    println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint |= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator |= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::max();
    a_biguint |= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator |= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::zero();
    a_biguint |= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator |= swallowed (took the ownership of) b_biguint.
    println!("---------------------------");
}

fn biguint_operator_xor()
{
    println!("biguint_operator_xor");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    
    println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    
    println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    
    println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let _c_biguint = a_biguint ^ b_biguint;
    // It cannot be compiled!
    // println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::max();
    let _c_biguint = a_biguint ^ b_biguint;
    // It cannot be compiled!
    // println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let _c_biguint = a_biguint ^ b_biguint;
    // It cannot be compiled!
    // println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    println!("---------------------------");
}

fn biguint_operator_xor_assign()
{
    println!("biguint_operator_xor_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint ^= b_biguint.clone();
    println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::max();
    a_biguint ^= b_biguint.clone();
    println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::zero();
    a_biguint ^= b_biguint.clone();
    println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    a_biguint ^= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::max();
    a_biguint ^= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^= swallowed (took the ownership of) b_biguint.

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    let b_biguint = U256::zero();
    a_biguint ^= b_biguint;
    // It cannot be compiled!
    // println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ^= swallowed (took the ownership of) b_biguint.
    println!("---------------------------");
}

fn biguint_operator_flip()
{
    println!("biguint_operator_flip");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = !a_biguint.clone();
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::max();
    let res = !a_biguint.clone();
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = !a_biguint.clone();
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It cannot be compiled!
    biguint_compile_fail_operator_flip();
    println!("---------------------------");
}

#[test]
fn biguint_compile_fail_operator_flip()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let _res = !a_biguint;

    #[cfg(compile_fail)]
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ! swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let _res = !a_biguint;
    
    #[cfg(compile_fail)]
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ! swallowed (took the ownership of) a_biguint.

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let _res = !a_biguint;
    
    #[cfg(compile_fail)]
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    // The operator ! swallowed (took the ownership of) a_biguint.
}



fn biguint_operators_miscellaneous_main()
{
    biguint_operator_eq();
    biguint_operator_lt();
    biguint_operator_gt();
    biguint_operator_le();
    biguint_operator_ge();
}

fn biguint_operator_eq()
{
    println!("biguint_operator_eq");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint == b_uint;
    if res
        { println!("{} == {}", a_biguint, b_uint); }
    else
        { println!("{} != {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint == b_uint;
    if res
        { println!("{} == {}", a_biguint, b_uint); }
    else
        { println!("{} != {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint != b_uint;
    if res
        { println!("{} != {}", a_biguint, b_uint); }
    else
        { println!("{} == {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint != b_uint;
    if res
        { println!("{} != {}", a_biguint, b_uint); }
    else
        { println!("{} == {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint == b_biguint;
    if res
        { println!("{} = {}", a_biguint, b_biguint); }
    else
        { println!("{} != {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint == b_biguint;
    if res
        { println!("{} = {}", a_biguint, b_biguint); }
    else
        { println!("{} != {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint != b_biguint;
    if res
        { println!("{} != {}", a_biguint, b_biguint); }
    else
        { println!("{} == {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint != b_biguint;
    if res
        { println!("{} != {}", a_biguint, b_biguint); }
    else
        { println!("{} == {}", a_biguint, b_biguint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_operator_lt()
{
    println!("biguint_operator_lt");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint < b_uint;
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint < b_uint;
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint < b_uint;
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, false);
    
    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint < b_biguint;
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint < b_biguint;
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint < b_biguint;
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_operator_gt()
{
    println!("biguint_operator_gt");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint > b_uint;
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint > b_uint;
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint > b_uint;
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, false);
    
    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint > b_biguint;
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint > b_biguint;
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint > b_biguint;
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_operator_le()
{
    println!("biguint_operator_le");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint <= b_uint;
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint <= b_uint;
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint <= b_uint;
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint <= b_biguint;
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint <= b_biguint;
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint <= b_biguint;
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_operator_ge()
{
    println!("biguint_operator_ge");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint >= b_uint;
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint >= b_uint;
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint >= b_uint;
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, true);
    
    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint >= b_biguint;
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint >= b_biguint;
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint >= b_biguint;
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, true);
    println!("---------------------------");
}


fn biguint_implementation_miscellaneous_main()
{
    biguint_display_fmt_for_biguint();
    biguint_upperhex_fmt_for_biguint();
    biguint_lowerhex_fmt_for_biguint();
    biguint_binary_fmt_for_biguint();
    biguint_octal_fmt_for_biguint();
    biguint_upperexp_fmt_for_biguint();
    biguint_lowerexp_fmt_for_biguint();
    biguint_pointer_fmt_for_biguint();
    biguint_from();
    biguint_from_str();
    biguint_number_err();
    biguint_display_fmt_for_numbererr();
}

fn biguint_display_fmt_for_biguint()
{
    println!("biguint_display_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{}", a_biguint);
    assert_eq!(a_biguint.to_string(), "69743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>80}", a_biguint);
    let txt = format!("{:>80}", a_biguint);
    assert_eq!(txt, "   69743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>080}", a_biguint);
    let txt = format!("{:>080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^80}", a_biguint);
    let txt = format!("{:^80}", a_biguint);
    assert_eq!(txt, " 69743176821145534028236692093846345739169743176821145534028236692093846345739  ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^080}", a_biguint);
    let txt = format!("{:^080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<80}", a_biguint);
    let txt = format!("{:<80}", a_biguint);
    assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739   ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<080}", a_biguint);
    let txt = format!("{:<080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>80}", a_biguint);
    let txt = format!("{:!>80}", a_biguint);
    assert_eq!(txt, "!!!69743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>080}", a_biguint);
    let txt = format!("{:@>080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^80}", a_biguint);
    let txt = format!("{:#^80}", a_biguint);
    assert_eq!(txt, "#69743176821145534028236692093846345739169743176821145534028236692093846345739##");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^080}", a_biguint);
    let txt = format!("{:$^080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<80}", a_biguint);
    let txt = format!("{:%<80}", a_biguint);
    assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^<080}", a_biguint);
    let txt = format!("{:^<080}", a_biguint);
    assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");

    let a_biguint = U256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    let txt = a_biguint.to_string();
    println!("{}", txt);
    assert_eq!(txt, "12345671234567890123456789012345678901234567890123456789012345678901234567890");
    println!("---------------------------");
}


fn biguint_upperhex_fmt_for_biguint()
{
    println!("biguint_upperhex_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:X}", a_biguint);
    let txt = format!("{:X}", a_biguint);
    assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:80X}", a_biguint);
    let txt = format!("{:80X}", a_biguint);
    assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:080X}", a_biguint);
    let txt = format!("{:080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#X}", a_biguint);
    let txt = format!("{:#X}", a_biguint);
    assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#80X}", a_biguint);
    let txt = format!("{:#80X}", a_biguint);
    assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#080X}", a_biguint);
    let txt = format!("{:#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>80X}", a_biguint);
    let txt = format!("{:>80X}", a_biguint);
    assert_eq!(txt, "                9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>080X}", a_biguint);
    let txt = format!("{:>080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^80X}", a_biguint);
    let txt = format!("{:^80X}", a_biguint);
    assert_eq!(txt, "        9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B        ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^080X}", a_biguint);
    let txt = format!("{:^080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<80X}", a_biguint);
    let txt = format!("{:<80X}", a_biguint);
    assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<080X}", a_biguint);
    let txt = format!("{:<080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>80X}", a_biguint);
    let txt = format!("{:!>80X}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!!!9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>080X}", a_biguint);
    let txt = format!("{:@>080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^80X}", a_biguint);
    let txt = format!("{:#^80X}", a_biguint);
    assert_eq!(txt, "########9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B########");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^080X}", a_biguint);
    let txt = format!("{:$^080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<80X}", a_biguint);
    let txt = format!("{:%<80X}", a_biguint);
    assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B%%%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:*<080X}", a_biguint);
    let txt = format!("{:*<080X}", a_biguint);
    assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#80X}", a_biguint);
    let txt = format!("{:>#80X}", a_biguint);
    assert_eq!(txt, "              0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#080X}", a_biguint);
    let txt = format!("{:>#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#80X}", a_biguint);
    let txt = format!("{:^#80X}", a_biguint);
    assert_eq!(txt, "       0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B       ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#080X}", a_biguint);
    let txt = format!("{:^#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#80X}", a_biguint);
    let txt = format!("{:<#80X}", a_biguint);
    assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#080X}", a_biguint);
    let txt = format!("{:<#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>#80X}", a_biguint);
    let txt = format!("{:!>#80X}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>#080X}", a_biguint);
    let txt = format!("{:@>#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^#80X}", a_biguint);
    let txt = format!("{:#^#80X}", a_biguint);
    assert_eq!(txt, "#######0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B#######");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^#080X}", a_biguint);
    let txt = format!("{:$^#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<#80X}", a_biguint);
    let txt = format!("{:%<#80X}", a_biguint);
    assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^<#080X}", a_biguint);
    let txt = format!("{:^<#080X}", a_biguint);
    assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    println!("---------------------------");
}

fn biguint_lowerhex_fmt_for_biguint()
{
    println!("biguint_lowerhex_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:x}", a_biguint);
    let txt = format!("{:x}", a_biguint);
    assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:80x}", a_biguint);
    let txt = format!("{:80x}", a_biguint);
    assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:080x}", a_biguint);
    let txt = format!("{:080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#x}", a_biguint);
    let txt = format!("{:#x}", a_biguint);
    assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#80x}", a_biguint);
    let txt = format!("{:#80x}", a_biguint);
    assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#080x}", a_biguint);
    let txt = format!("{:#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>80x}", a_biguint);
    let txt = format!("{:>80x}", a_biguint);
    assert_eq!(txt, "                9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>080x}", a_biguint);
    let txt = format!("{:>080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^80x}", a_biguint);
    let txt = format!("{:^80x}", a_biguint);
    assert_eq!(txt, "        9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b        ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^080x}", a_biguint);
    let txt = format!("{:^080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<80x}", a_biguint);
    let txt = format!("{:<80x}", a_biguint);
    assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<080x}", a_biguint);
    let txt = format!("{:<080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>80x}", a_biguint);
    let txt = format!("{:!>80x}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!!!9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>080x}", a_biguint);
    let txt = format!("{:@>080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^80x}", a_biguint);
    let txt = format!("{:#^80x}", a_biguint);
    assert_eq!(txt, "########9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b########");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^080x}", a_biguint);
    let txt = format!("{:$^080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<80x}", a_biguint);
    let txt = format!("{:%<80x}", a_biguint);
    assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b%%%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:*<080x}", a_biguint);
    let txt = format!("{:*<080x}", a_biguint);
    assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#80x}", a_biguint);
    let txt = format!("{:>#80x}", a_biguint);
    assert_eq!(txt, "              0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#080x}", a_biguint);
    let txt = format!("{:>#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#80x}", a_biguint);
    let txt = format!("{:^#80x}", a_biguint);
    assert_eq!(txt, "       0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b       ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#080x}", a_biguint);
    let txt = format!("{:^#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#80x}", a_biguint);
    let txt = format!("{:<#80x}", a_biguint);
    assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#080x}", a_biguint);
    let txt = format!("{:<#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>#80x}", a_biguint);
    let txt = format!("{:!>#80x}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>#080x}", a_biguint);
    let txt = format!("{:@>#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^#80x}", a_biguint);
    let txt = format!("{:#^#80x}", a_biguint);
    assert_eq!(txt, "#######0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b#######");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^#080x}", a_biguint);
    let txt = format!("{:$^#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<#80x}", a_biguint);
    let txt = format!("{:%<#80x}", a_biguint);
    assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^<#080x}", a_biguint);
    let txt = format!("{:^<#080x}", a_biguint);
    assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    println!("---------------------------");
}

fn biguint_binary_fmt_for_biguint()
{
    println!("biguint_binary_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:b}", a_biguint);
    let txt = format!("{:b}", a_biguint);
    assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap(); 
    println!("{:272b}", a_biguint);
    let txt = format!("{:272b}", a_biguint);
    assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011                ");
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:0272b}", a_biguint);
    let txt = format!("{:0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#b}", a_biguint);
    let txt = format!("{:#b}", a_biguint);
    assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#272b}", a_biguint);
    let txt = format!("{:#272b}", a_biguint);
    assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#0272b}", a_biguint);
    let txt = format!("{:#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>272b}", a_biguint);
    let txt = format!("{:>272b}", a_biguint);
    assert_eq!(txt, "                1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>0272b}", a_biguint);
    let txt = format!("{:>0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^272b}", a_biguint);
    let txt = format!("{:^272b}", a_biguint);
    assert_eq!(txt, "        1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011        ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^0272b}", a_biguint);
    let txt = format!("{:^0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<272b}", a_biguint);
    let txt = format!("{:<272b}", a_biguint);
    assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<0272b}", a_biguint);
    let txt = format!("{:<0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>272b}", a_biguint);
    let txt = format!("{:!>272b}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!!!1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>0272b}", a_biguint);
    let txt = format!("{:@>0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^272b}", a_biguint);
    let txt = format!("{:#^272b}", a_biguint);
    assert_eq!(txt, "########1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011########");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^0272b}", a_biguint);
    let txt = format!("{:$^0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<272b}", a_biguint);
    let txt = format!("{:%<272b}", a_biguint);
    assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011%%%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:*<0272b}", a_biguint);
    let txt = format!("{:*<0272b}", a_biguint);
    assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#272b}", a_biguint);
    let txt = format!("{:>#272b}", a_biguint);
    assert_eq!(txt, "              0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#0272b}", a_biguint);
    let txt = format!("{:>#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#272b}", a_biguint);
    let txt = format!("{:^#272b}", a_biguint);
    assert_eq!(txt, "       0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011       ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#0272b}", a_biguint);
    let txt = format!("{:^#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#272b}", a_biguint);
    let txt = format!("{:<#272b}", a_biguint);
    assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#0272b}", a_biguint);
    let txt = format!("{:<#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>#272b}", a_biguint);
    let txt = format!("{:!>#272b}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>#0272b}", a_biguint);
    let txt = format!("{:@>#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^#272b}", a_biguint);
    let txt = format!("{:#^#272b}", a_biguint);
    assert_eq!(txt, "#######0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011#######");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^#0272b}", a_biguint);
    let txt = format!("{:$^#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<#272b}", a_biguint);
    let txt = format!("{:%<#272b}", a_biguint);
    assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^<#0272b}", a_biguint);
    let txt = format!("{:^<#0272b}", a_biguint);
    assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    println!("---------------------------");
}

fn biguint_octal_fmt_for_biguint()
{
    println!("biguint_octal_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:o}", a_biguint);
    let txt = format!("{:o}", a_biguint);
    assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap(); 
    println!("{:102o}", a_biguint);
    let txt = format!("{:102o}", a_biguint);
    assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:0102o}", a_biguint);
    let txt = format!("{:0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#o}", a_biguint);
    let txt = format!("{:#o}", a_biguint);
    assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#102o}", a_biguint);
    let txt = format!("{:#102o}", a_biguint);
    assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#0102o}", a_biguint);
    let txt = format!("{:#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>102o}", a_biguint);
    let txt = format!("{:>102o}", a_biguint);
    assert_eq!(txt, "                11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>0102o}", a_biguint);
    let txt = format!("{:>0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^102o}", a_biguint);
    let txt = format!("{:^102o}", a_biguint);
    assert_eq!(txt, "        11506117236125542215231050004463337025330420704216361264762044667420601552357042554013        ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^0102o}", a_biguint);
    let txt = format!("{:^0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<102o}", a_biguint);
    let txt = format!("{:<102o}", a_biguint);
    assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<0102o}", a_biguint);
    let txt = format!("{:<0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>102o}", a_biguint);
    let txt = format!("{:!>102o}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!!!11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>0102o}", a_biguint);
    let txt = format!("{:@>0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^102o}", a_biguint);
    let txt = format!("{:#^102o}", a_biguint);
    assert_eq!(txt, "########11506117236125542215231050004463337025330420704216361264762044667420601552357042554013########");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^0102o}", a_biguint);
    let txt = format!("{:$^0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<102o}", a_biguint);
    let txt = format!("{:%<102o}", a_biguint);
    assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:*<0102o}", a_biguint);
    let txt = format!("{:*<0102o}", a_biguint);
    assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#102o}", a_biguint);
    let txt = format!("{:>#102o}", a_biguint);
    assert_eq!(txt, "              0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:>#0102o}", a_biguint);
    let txt = format!("{:>#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#102o}", a_biguint);
    let txt = format!("{:^#102o}", a_biguint);
    assert_eq!(txt, "       0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013       ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^#0102o}", a_biguint);
    let txt = format!("{:^#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#102o}", a_biguint);
    let txt = format!("{:<#102o}", a_biguint);
    assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:<#0102o}", a_biguint);
    let txt = format!("{:<#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    
    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:!>#102o}", a_biguint);
    let txt = format!("{:!>#102o}", a_biguint);
    assert_eq!(txt, "!!!!!!!!!!!!!!0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:@>#0102o}", a_biguint);
    let txt = format!("{:@>#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:#^#102o}", a_biguint);
    let txt = format!("{:#^#102o}", a_biguint);
    assert_eq!(txt, "#######0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013#######");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:$^#0102o}", a_biguint);
    let txt = format!("{:$^#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:%<#102o}", a_biguint);
    let txt = format!("{:%<#102o}", a_biguint);
    assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%");

    let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    println!("{:^<#0102o}", a_biguint);
    let txt = format!("{:^<#0102o}", a_biguint);
    assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    println!("---------------------------");
}

fn biguint_upperexp_fmt_for_biguint()
{
    println!("biguint_upperexp_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:E}", a_biguint);
    let txt = format!("{:E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:100E}", a_biguint);
    let txt = format!("{:100E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:0100E}", a_biguint);
    let txt = format!("{:0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:20.9E}", a_biguint);
    let txt = format!("{:20.9E}", a_biguint);
    assert_eq!(txt, "1.234567890E76      ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:020.9E}", a_biguint);
    let txt = format!("{:020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:21.13E}", a_biguint);
    let txt = format!("{:21.13E}", a_biguint);
    assert_eq!(txt, "1.2345678901235E76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:021.13E}", a_biguint);
    let txt = format!("{:021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<E}", a_biguint);
    let txt = format!("{:<E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<100E}", a_biguint);
    let txt = format!("{:<100E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<0100E}", a_biguint);
    let txt = format!("{:<0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<20.9E}", a_biguint);
    let txt = format!("{:<20.9E}", a_biguint);
    assert_eq!(txt, "1.234567890E76      ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<020.9E}", a_biguint);
    let txt = format!("{:<020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<21.13E}", a_biguint);
    let txt = format!("{:<21.13E}", a_biguint);
    assert_eq!(txt, "1.2345678901235E76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<021.13E}", a_biguint);
    let txt = format!("{:<021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!<E}", a_biguint);
    let txt = format!("{:<E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@<100E}", a_biguint);
    let txt = format!("{:@<100E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@@@@@@@@@@");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#<0100E}", a_biguint);
    let txt = format!("{:#<0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$<20.9E}", a_biguint);
    let txt = format!("{:$<20.9E}", a_biguint);
    assert_eq!(txt, "1.234567890E76$$$$$$");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%<020.9E}", a_biguint);
    let txt = format!("{:%<020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^<21.13E}", a_biguint);
    let txt = format!("{:^<21.13E}", a_biguint);
    assert_eq!(txt, "1.2345678901235E76^^^");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&<021.13E}", a_biguint);
    let txt = format!("{:&<021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>E}", a_biguint);
    let txt = format!("{:>E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>100E}", a_biguint);
    let txt = format!("{:>100E}", a_biguint);
    assert_eq!(txt, "                   1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>0100E}", a_biguint);
    let txt = format!("{:>0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>20.9E}", a_biguint);
    let txt = format!("{:>20.9E}", a_biguint);
    assert_eq!(txt, "      1.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>020.9E}", a_biguint);
    let txt = format!("{:>020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>21.13E}", a_biguint);
    let txt = format!("{:>21.13E}", a_biguint);
    assert_eq!(txt, "   1.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>021.13E}", a_biguint);
    let txt = format!("{:>021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!>E}", a_biguint);
    let txt = format!("{:>E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@>100E}", a_biguint);
    let txt = format!("{:@>100E}", a_biguint);
    assert_eq!(txt, "@@@@@@@@@@@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#>0100E}", a_biguint);
    let txt = format!("{:#>0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$>20.9E}", a_biguint);
    let txt = format!("{:$>20.9E}", a_biguint);
    assert_eq!(txt, "$$$$$$1.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%>020.9E}", a_biguint);
    let txt = format!("{:%>020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^>21.13E}", a_biguint);
    let txt = format!("{:^>21.13E}", a_biguint);
    assert_eq!(txt, "^^^1.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&>021.13E}", a_biguint);
    let txt = format!("{:&>021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");
    
    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^E}", a_biguint);
    let txt = format!("{:^E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^100E}", a_biguint);
    let txt = format!("{:^100E}", a_biguint);
    assert_eq!(txt, "         1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76          ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^0100E}", a_biguint);
    let txt = format!("{:^0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^20.9E}", a_biguint);
    let txt = format!("{:^20.9E}", a_biguint);
    assert_eq!(txt, "   1.234567890E76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^020.9E}", a_biguint);
    let txt = format!("{:^020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^21.13E}", a_biguint);
    let txt = format!("{:^21.13E}", a_biguint);
    assert_eq!(txt, " 1.2345678901235E76  ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^021.13E}", a_biguint);
    let txt = format!("{:^021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!^E}", a_biguint);
    let txt = format!("{:^E}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@^100E}", a_biguint);
    let txt = format!("{:@^100E}", a_biguint);
    assert_eq!(txt, "@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#^0100E}", a_biguint);
    let txt = format!("{:#^0100E}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$^20.9E}", a_biguint);
    let txt = format!("{:$^20.9E}", a_biguint);
    assert_eq!(txt, "$$$1.234567890E76$$$");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%^020.9E}", a_biguint);
    let txt = format!("{:%^020.9E}", a_biguint);
    assert_eq!(txt, "0000001.234567890E76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^^21.13E}", a_biguint);
    let txt = format!("{:^^21.13E}", a_biguint);
    assert_eq!(txt, "^1.2345678901235E76^^");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&^021.13E}", a_biguint);
    let txt = format!("{:&^021.13E}", a_biguint);
    assert_eq!(txt, "0001.2345678901235E76");
    println!("---------------------------");
}
fn biguint_lowerexp_fmt_for_biguint()
{
    println!("biguint_lowerexp_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:e}", a_biguint);
    let txt = format!("{:e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:100e}", a_biguint);
    let txt = format!("{:100e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76                   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:0100e}", a_biguint);
    let txt = format!("{:0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:20.9e}", a_biguint);
    let txt = format!("{:20.9e}", a_biguint);
    assert_eq!(txt, "1.234567890e76      ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:020.9e}", a_biguint);
    let txt = format!("{:020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:21.13e}", a_biguint);
    let txt = format!("{:21.13e}", a_biguint);
    assert_eq!(txt, "1.2345678901235e76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:021.13e}", a_biguint);
    let txt = format!("{:021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<e}", a_biguint);
    let txt = format!("{:<e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<100e}", a_biguint);
    let txt = format!("{:<100e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76                   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<0100e}", a_biguint);
    let txt = format!("{:<0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<20.9e}", a_biguint);
    let txt = format!("{:<20.9e}", a_biguint);
    assert_eq!(txt, "1.234567890e76      ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<020.9e}", a_biguint);
    let txt = format!("{:<020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<21.13e}", a_biguint);
    let txt = format!("{:<21.13e}", a_biguint);
    assert_eq!(txt, "1.2345678901235e76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<021.13e}", a_biguint);
    let txt = format!("{:<021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!<e}", a_biguint);
    let txt = format!("{:<e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@<100e}", a_biguint);
    let txt = format!("{:@<100e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76@@@@@@@@@@@@@@@@@@@");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#<0100e}", a_biguint);
    let txt = format!("{:#<0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$<20.9e}", a_biguint);
    let txt = format!("{:$<20.9e}", a_biguint);
    assert_eq!(txt, "1.234567890e76$$$$$$");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%<020.9e}", a_biguint);
    let txt = format!("{:%<020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^<21.13e}", a_biguint);
    let txt = format!("{:^<21.13e}", a_biguint);
    assert_eq!(txt, "1.2345678901235e76^^^");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&<021.13e}", a_biguint);
    let txt = format!("{:&<021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>e}", a_biguint);
    let txt = format!("{:>e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>100e}", a_biguint);
    let txt = format!("{:>100e}", a_biguint);
    assert_eq!(txt, "                   1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>0100e}", a_biguint);
    let txt = format!("{:>0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>20.9e}", a_biguint);
    let txt = format!("{:>20.9e}", a_biguint);
    assert_eq!(txt, "      1.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>020.9e}", a_biguint);
    let txt = format!("{:>020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>21.13e}", a_biguint);
    let txt = format!("{:>21.13e}", a_biguint);
    assert_eq!(txt, "   1.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>021.13e}", a_biguint);
    let txt = format!("{:>021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!>e}", a_biguint);
    let txt = format!("{:>e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@>100e}", a_biguint);
    let txt = format!("{:@>100e}", a_biguint);
    assert_eq!(txt, "@@@@@@@@@@@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#>0100e}", a_biguint);
    let txt = format!("{:#>0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$>20.9e}", a_biguint);
    let txt = format!("{:$>20.9e}", a_biguint);
    assert_eq!(txt, "$$$$$$1.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%>020.9e}", a_biguint);
    let txt = format!("{:%>020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^>21.13e}", a_biguint);
    let txt = format!("{:^>21.13e}", a_biguint);
    assert_eq!(txt, "^^^1.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&>021.13e}", a_biguint);
    let txt = format!("{:&>021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");
    
    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^e}", a_biguint);
    let txt = format!("{:^e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^100e}", a_biguint);
    let txt = format!("{:^100e}", a_biguint);
    assert_eq!(txt, "         1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76          ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^0100e}", a_biguint);
    let txt = format!("{:^0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^20.9e}", a_biguint);
    let txt = format!("{:^20.9e}", a_biguint);
    assert_eq!(txt, "   1.234567890e76   ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^020.9e}", a_biguint);
    let txt = format!("{:^020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^21.13e}", a_biguint);
    let txt = format!("{:^21.13e}", a_biguint);
    assert_eq!(txt, " 1.2345678901235e76  ");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^021.13e}", a_biguint);
    let txt = format!("{:^021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!^e}", a_biguint);
    let txt = format!("{:^e}", a_biguint);
    assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@^100e}", a_biguint);
    let txt = format!("{:@^100e}", a_biguint);
    assert_eq!(txt, "@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76@@@@@@@@@@");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#^0100e}", a_biguint);
    let txt = format!("{:#^0100e}", a_biguint);
    assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$^20.9e}", a_biguint);
    let txt = format!("{:$^20.9e}", a_biguint);
    assert_eq!(txt, "$$$1.234567890e76$$$");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%^020.9e}", a_biguint);
    let txt = format!("{:%^020.9e}", a_biguint);
    assert_eq!(txt, "0000001.234567890e76");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^^21.13e}", a_biguint);
    let txt = format!("{:^^21.13e}", a_biguint);
    assert_eq!(txt, "^1.2345678901235e76^^");

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&^021.13e}", a_biguint);
    let txt = format!("{:&^021.13e}", a_biguint);
    assert_eq!(txt, "0001.2345678901235e76");
    println!("---------------------------");
}
fn biguint_pointer_fmt_for_biguint()
{
    println!("biguint_pointer_fmt_for_biguint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:p}", a_biguint);
    let txt = format!("{:p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958aab0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:20p}", a_biguint);
    let txt = format!("{:20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958b0b0      "); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:020p}", a_biguint);
    let txt = format!("{:020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958aae0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<p}", a_biguint);
    let txt = format!("{:<p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958b0e0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<20p}", a_biguint);
    let txt = format!("{:<20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958ab10      "); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:<020p}", a_biguint);
    let txt = format!("{:<020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958b1a0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:!<p}", a_biguint);
    let txt = format!("{:!<p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958ab40"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:@<20p}", a_biguint);
    let txt = format!("{:@<20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958b1d0@@@@@@"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:#<020p}", a_biguint);
    let txt = format!("{:#<020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958ab70"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>p}", a_biguint);
    let txt = format!("{:>p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958b200"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>20p}", a_biguint);
    let txt = format!("{:>20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "      0x7ffcd958aba0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:>020p}", a_biguint);
    let txt = format!("{:>020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958b230"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:$>p}", a_biguint);
    let txt = format!("{:$>p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958abd0"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:%>20p}", a_biguint);
    let txt = format!("{:%>20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "%%%%%%0x7ffcd958b110"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^>020p}", a_biguint);
    let txt = format!("{:^>020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "00x0000007ffcd958a750"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^p}", a_biguint);
    let txt = format!("{:^p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958a850"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^20p}", a_biguint);
    let txt = format!("{:^20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "   0x7ffcd958b140   "); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:^020p}", a_biguint);
    let txt = format!("{:^020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958b170"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:&^p}", a_biguint);
    let txt = format!("{:&^p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x7ffcd958af40"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:*^20p}", a_biguint);
    let txt = format!("{:*^20p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "***0x7ffcd958af70***"); // can be different everytime

    let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    println!("{:_^020p}", a_biguint);
    let txt = format!("{:_^020p}", a_biguint);
    println!("{}", txt);
    // assert_eq!(txt, "0x0000007ffcd958afa0"); // can be different everytime
    println!("---------------------------");
}

fn biguint_from()
{
    println!("biguint_from()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from([1, 2, 3, 4, 5, 6, 7, 8]);
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    println!("---------------------------");
}

fn biguint_from_str()
{
    println!("biguint_from_str()");
    use std::str::FromStr;
    use cryptocol::number::NumberErr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint_wrapped = U256::from_str("215679573381144830513811895868694400695694534256768036697775454289921");
    match a_biguint_wrapped
    {
        Ok(a_biguint) => {
                println!("a_biguint = {}", a_biguint);
                assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
            },
        Err(e) => { println!("Error: {}", e); }
    }
    
    let a_biguint_wrapped = U256::from_str("@!#$%^&*()_+=-|-/?><`~");
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e, NumberErr::NotAlphaNumeric);
            }
    }

    let a_biguint_wrapped = "215679573381144830513811895868694400695694534256768036697775454289921".parse::<U256>();
    match a_biguint_wrapped
    {
        Ok(a_biguint) => {
                println!("a_biguint = {}", a_biguint);
                assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
            },
        Err(e) => { println!("Error: {}", e); }
    }

    let a_biguint_wrapped = "@!#$%^&*()_+=-|-/?><`~".parse::<U256>();
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e, NumberErr::NotAlphaNumeric);
            }
    }
    println!("---------------------------");
}

fn biguint_number_err()
{
    println!("biguint_number_err()");
    use std::str::FromStr;
    use cryptocol::number::NumberErr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 100);
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The given radix is out of the valid range. It should be in the range from binary up to 62-ary, inclusively.");
                assert_eq!(e, NumberErr::OutOfValidRadixRange);
            }
    }

    let a_biguint_wrapped = U256::from_str("@!#$%^&*()_+=-|-/?><`~");
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The string or the character is not alphanumeric.");
                assert_eq!(e, NumberErr::NotAlphaNumeric);
            }
    }
    
    let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 16);
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The string or the character is not fit to the given radix.");
                assert_eq!(e, NumberErr::NotFitToRadix);
            }
    }

    let a_biguint_wrapped = U256::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The number that the string represents is too big for the created object to contain.");
                assert_eq!(e, NumberErr::TooBigNumber);
            }
    }
    println!("---------------------------");
}

fn biguint_display_fmt_for_numbererr()
{
    println!("biguint_display_fmt_for_numbererr()");
    use cryptocol::number::NumberErr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 100);
    match a_biguint_wrapped
    {
        Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
        Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The given radix is out of the valid range. It should be in the range from binary up to 62-ary, inclusively.");
                assert_eq!(e, NumberErr::OutOfValidRadixRange);
            }
    }

    println!("NumberErr::NotAlphaNumeric: {}", NumberErr::NotAlphaNumeric);
    assert_eq!(NumberErr::NotAlphaNumeric.to_string(), "The string or the character is not alphanumeric.");

    let txt = NumberErr::TooBigNumber.to_string();
    println!("Error: {}", txt);
    assert_eq!(txt, "The number that the string represents is too big for the created object to contain.");

    let error = NumberErr::NotFitToRadix;
    println!("NumberErr::NotFitToRadix: {}", error);
    assert_eq!(NumberErr::NotFitToRadix.to_string(), "The string or the character is not fit to the given radix.");
    println!("---------------------------");
}


