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
    find_maximum();
    test();
    biguint_random_number_main();
}


pub fn find_maximum()
{
    println!("find_maximum()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_uint(123_u8);
    let mut exp = U256::one();
    loop {
        let b = a_biguint.pow(&exp);
        if b.is_overflow()
        {
            println!("Maximum i is {}", exp);
            break;
        }
        exp.wrapping_add_assign_uint(1_u8);
    }
    println!("---------------------------");
}

pub fn test()
{
    println!("test()");
    use cryptocol::number::*;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a = 128_u8;
    let b = a << 1;
    println!("b = {}", b);
    let p = U256::from_uint(12345678901234567890123456789_u128);
    let q = U256::from_uint(12345678901234567890_u128);
    let r = p.gcd(&q);

    println!("{} , {} => {}", p, q, r);

    let a_biguint = U256::from_uint(254_u8);
    let b = U256::from_uint(123_u8);
    let c = a_biguint.divide_fully(&b);
    let d = a_biguint.divide_fully_uint(123_u8);
    let aa = LongerUnion::new_with(254_u128);
    let bb = LongerUnion::new_with(123_u128);

    let cc = aa % bb;

    println!("c: {}  {}", c.0, c.1);
    println!("d: {}  {}", d.0, d.1);
    println!("{}", cc);

    let e = a_biguint.divide_fully_uint(4_u8);
    println!("{:?} {:?}", e.0, e.1);

    println!("a == b {}", a == b);
    println!("a != b {}", a != b);
    println!("a > b {}", a > b);
    println!("a >= b {}", a >= b);
    println!("a < b {}", a < b);
    println!("a <= b {}", a <= b);
}


fn biguint_random_number_main()
{
    biguint_any();
    biguint_any_odd();
    biguint_any_less_than();
    biguint_any_odd_less_than();
    biguint_any_with_MSB_set();
    biguint_any_odd_with_MSB_set();
    biguint_any_prime_using_Miller_Rabin();
    biguint_turn_any();
    biguint_random();
    biguint_random_odd();
    biguint_random_less_than();
    biguint_random_odd_less_than();
    biguint_random_with_MSB_set();
    biguint_random_odd_with_MSB_set();
    biguint_random_prime_using_Miller_Rabin();
    biguint_randomize();
    biguint_is_prime_using_miller_rabin();
}

fn biguint_any()
{
    println!("biguint_any");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    println!("Random Number: {}", U1024::any());
    println!("---------------------------");
}

fn biguint_any_odd()
{
    println!("biguint_any_odd");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let r = U1024::any_odd();
    println!("Random Odd Number: {}", r);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn biguint_any_less_than()
{
    println!("biguint_any_less_than");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let ceiling = U1024::max() / U1024::from_uint::<u32>(3);
    let r = U1024::any_less_than(&ceiling);
    println!("Random Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    println!("---------------------------");
}

fn biguint_any_odd_less_than()
{
    println!("biguint_any_odd_less_than");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    let ceiling = U1024::max() / U1024::from_uint::<u32>(3);
    let r = U1024::any_odd_less_than(&ceiling);
    println!("Random Odd Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn biguint_any_with_MSB_set()
{
    println!("biguint_any_with_MSB_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let num = U1024::any_with_MSB_set();
    println!("Random Number = {}", U1024::any());
    println!("1024-bit Random Number = {}", num);
    assert!(num > U1024::submax(1023));
    println!("---------------------------");
}

fn biguint_any_odd_with_MSB_set()
{
    println!("biguint_any_odd_with_MSB_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let num = U1024::any_odd_with_MSB_set();
    println!("Random Number = {}", U1024::any());
    println!("1024-bit Random Odd Number = {}", num);
    assert!(num > U1024::submax(1023));
    assert!(num.is_odd());
    println!("---------------------------");
}

fn biguint_any_prime_using_Miller_Rabin()
{
    println!("biguint_any_prime_using_Miller_Rabin");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let num = U256::any_prime_using_Miller_Rabin(5);
    println!("Random Prime Number = {}", num);
    assert!(num.is_prime_using_miller_rabin(5));
    println!("---------------------------");
    
}

fn biguint_turn_any()
{
    println!("biguint_turn_any");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let mut r = U256::new();
    println!("original number = {}", r);
    assert_eq!(r, U256::zero());
    r.turn_any();
    println!("random number = {}", r);
    assert_ne!(r, U256::zero());
    println!("---------------------------");
}


fn biguint_random()
{
    println!("biguint_random");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    println!("Random Number: {}", U1024::random());
    println!("---------------------------");
}

fn biguint_random_odd()
{
    println!("biguint_random_odd");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let r = U1024::random_odd();
    println!("Random Odd Number: {}", r);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn biguint_random_less_than()
{
    println!("biguint_random_less_than");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    let ceiling = U1024::max() / U1024::from_uint::<u32>(3);
    let r = U1024::random_less_than(&ceiling);
    println!("Random Number less than {} is {}", ceiling, r);
    assert!(r < ceiling);
    println!("---------------------------");
}

fn biguint_random_odd_less_than()
{
    println!("biguint_random_odd_less_than");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let ceiling = U1024::max() / U1024::from_uint::<u32>(3);
    let r = U1024::random_odd_less_than(&ceiling);
    println!("Random Odd Number less than {} is {}", ceiling, U1024::random_odd_less_than(&ceiling));
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("---------------------------");
}

fn biguint_random_with_MSB_set()
{
    println!("biguint_random_with_MSB_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let num = U1024::random_with_MSB_set();
    println!("Random Number = {}", U1024::random());
    println!("1024-bit Random Number = {}", num);
    assert!(num > U1024::submax(1023));
    println!("---------------------------");
}

fn biguint_random_odd_with_MSB_set()
{
    println!("biguint_random_odd_with_MSB_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    let num = U1024::random_odd_with_MSB_set();
    println!("Random Number = {}", U1024::random());
    println!("1024-bit Random Odd Number = {}", num);
    assert!(num > U1024::submax(1023));
    assert!(num.is_odd());
    println!("---------------------------");
}

fn biguint_random_prime_using_Miller_Rabin()
{
    println!("biguint_random_prime_using_Miller_Rabin");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    let num = U1024::random_prime_using_Miller_Rabin(5);
    assert!(num.is_prime_using_miller_rabin(5));
    println!("Random Prime Number = {}", num);
    println!("---------------------------");
}

fn biguint_randomize()
{
    println!("biguint_randomize");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut r = U1024::new();
    println!("original number = {}", r);
    assert_eq!(r, U1024::zero());
    r.randomize();
    println!("random number = {}", r);
    assert_ne!(r, U1024::zero());
    println!("---------------------------");
}

fn biguint_is_prime_using_miller_rabin()
{
    println!("biguint_is_prime_using_miller_rabin");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num = U1024::from_string("157847659859841049478697210209054499132116730052547470511818639401226705057924429751936169954758794979780692256039595351594450957429818931145981533862363167515145703012676459279601554094177152095755375227908501443524236048737351327752857335149319939532219166843564206337168180636940438709755340632429325500479").unwrap();
    let yes = num.is_prime_using_miller_rabin(5);
    println!("Is {} a prime number? => {}", num, yes);
    if yes  { assert!(yes); }
    else    { assert!(!yes); }
    println!("---------------------------");
}


/*
fn f()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    let divisor = 87_u8;
    let dividend = 1234567890157589425462369896584689254_u128;
    let dd = U256::from_uint(dividend);
    let (quotient, remainder) = dd.divide_fully_uint(divisor);
    println!("{} - {}", quotient, remainder);
    let (quotient, remainder) = dd.divide_fully(&U256::from_uint(divisor));
    println!("{} - {}", quotient, remainder);

}

fn t_1024()
{
    define_utypes_with!(u128);
    let a_biguint = U1024::random();
    println!("{} 비트짜리 난수: {}", 1024, a);
    let b = U1024::from(1_u128);
    println!("{} 비트짜리 1: {}", 1024, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_2048()
{
    define_utypes_with!(u128);
    let a_biguint = U2048::random();
    println!("{} 비트짜리 난수: {}", 2048, a);
    let b = U2048::from(1_u128);
    println!("{} 비트짜리 1: {}", 2048, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_4096()
{
    define_utypes_with!(u128);
    let a_biguint = U4096::random();
    println!("{} 비트짜리 난수: {}", 4096, a);
    let b = U4096::from(1_u128);
    println!("{} 비트짜리 1: {}", 4096, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}



fn func<T: Uint + Add<Output = T>>(lhs: T, rhs: T) -> T
{
    lhs + rhs
}
fn func2<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn main()
{



    let a = 100;
    let b = a % -3;
    let c = "123456789012".parse::<U256>().unwrap();
    let e = c.to_string_with_radix_and_stride(10, 4);
    let d: u128 = c.into_u128();
    println!("a = {}, b = {}, c = {}, e = {}", a, b, c, e);
    let a = "123_4566".parse::<U256>().unwrap();
    println!("a = {}", a);
    let ss = UShort { byte: [101, 100] };
    unsafe { println!("ss.short = {}", ss.ushort ); }
    println!("{}", (25700_u16 + 25800_u16));

    // a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    let a_high = 100_u8;
    let a_low = 101_u8;
    // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    let b_high = 100_u8;
    let b_low = 200_u8;
    // c: u16 === (c_high, c_low)
    let c_high: u8;
    let c_low: u8;
    let mut carry: bool;
    // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    println!("{}-{}, {}", c_high, c_low, carry);
    assert_eq!(c_high, 201);
    assert_eq!(c_low, 45);
    assert_eq!(carry, false);

    let d_high: u128;
    let d_low: u128;
    let e = BigUInt::<u128, 2>::from_array(&[6789012345678919134, 12345678901234569124]);
    println!("big = {}", e);
    (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    println!("{}-{}, {}", d_high, d_low, carry);
    assert_eq!(d_high, 12345678901234569124);
    assert_eq!(d_low, 6789012345678919134);
    assert_eq!(carry, false);
}

fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
{
    let mut carry = false;
    let mut sum_high: T;
    let mut sum_low: T;
    (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_high, sum_low, carry)
}

fn main()
{
    let a = func(50_u128, 4_u128);
    println!("50 + 4 = {}", a);
    assert_eq!(a, 54_u128);

    let b = func2(u8::MAX, u8::MAX);
    println!("{} * 15_u64 = {}", u128::MAX, b);
    assert_eq!(b, 254_u8);
    
    // U256::new();
    // let a = 100_u8;
    // let b = 100_u8;
    // let c = func(a, b);
    // let d = func(c, 57);
    // println!("a + b = {}", c);
    // println!("c + 57 = {}", d);
    // assert_eq!(c, 200_u8);
    // assert_eq!(d, 1_u8);
    
    let mut a_biguint = U256::from_string_with_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let b = U256::from_string_with_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    let d = U256::max();
    let c = !a_biguint | a_biguint;
    println!("c = {}", c.to_string_with_radix(2));
    assert_eq!(c, U256::max());

    // let mut sum = U1024::new();
    // sum.set_max();
    // println!("sum = {}", sum);

    // let mut a_biguint = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    // println!("{}", a_biguint);
    // a_biguint >>= 2;
    // println!("a_biguint = {}\n{}", a_biguint, a_biguint.is_underflow());
    // assert_eq!(a_biguint.is_underflow(), true);
}
*/