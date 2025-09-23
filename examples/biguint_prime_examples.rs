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
    biguint_prime_quick_start();
    biguint_prime_gcd_uint();
    biguint_prime_gcd_assign_uint();
    biguint_prime_lcm_uint();
    biguint_prime_lcm_assign_uint();
    biguint_prime_gcd();
    biguint_prime_gcd_assign();
    biguint_prime_extended_gcd();
    // biguint_prime_lcm();
    // biguint_prime_lcm_assign();
    // biguint_prime_is_prime_using_miller_rabin();
}


fn biguint_prime_quick_start()
{
    println!("biguint_prime_quick_start()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u32);

    let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    let c_biguint = a_biguint.gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "27");

    let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    let c_biguint = a_biguint.lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");

    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    assert_eq!(b, true);
    println!("---------------------------");
}


fn biguint_prime_gcd_uint()
{
    println!("biguint_prime_gcd_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u32);

    // normal case
    let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    let c_biguint = a_biguint.gcd_uint(b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Panic Examples
    let _a_biguint = U256::zero();
    let _b_biguint = 103778310992036469625452733331446377109_u128;
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);

    let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = 0_u128;
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);

    let _a_biguint = U256::zero();
    let _b_biguint = 0_u128;
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_gcd_assign_uint()
{
    println!("biguint_prime_gcd_assign_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u64);

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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign_uint(b_biguint);
    println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic Examples
    let mut _a_biguint = U256::zero();
    let _b_biguint = 103778310992036469625452733331446377109_u128;
    // It will panic!
    // _a_biguint.gcd_assign_uint(_b_biguint);

    let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = 0_u128;
    // It will panic!
    // _a_biguint.gcd_assign_uint(_b_biguint);

    let mut _a_biguint = U256::zero();
    let _b_biguint = 0_u128;
    // It will panic!
    // _a_biguint.gcd_assign_uint(_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_lcm_uint()
{
    println!("biguint_prime_lcm_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u16);

    // normal case
    let a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    let c_biguint = a_biguint.lcm_uint(b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "20596479741978911975639783055646618200359178304364816695371910650463951431749917999104000000000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Panic Examples
    let _a_biguint = U256::zero();
    let _b_biguint = 103778310992036469625452733331446377109_u128;
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);

    let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = 0_u128;
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);

    let _a_biguint = U256::zero();
    let _b_biguint = 0_u128;
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_lcm_assign_uint()
{
    println!("biguint_prime_lcm_assign_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u32);

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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 103778310992036469625452733331446377109_u128;    assert_eq!(a_biguint.is_overflow(), false);
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign_uint(b_biguint);
    println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "20596479741978911975639783055646618200359178304364816695371910650463951431749917999104000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic Examples
    let mut _a_biguint = U256::zero();
    let _b_biguint = 103778310992036469625452733331446377109_u128;
    // It will panic!
    // _a_biguint.lcm_assign_uint(_b_biguint);

    let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = 0_u128;
    // It will panic!
    // _a_biguint.lcm_assign_uint(_b_biguint);

    let mut _a_biguint = U256::zero();
    let _b_biguint = 0_u128;
    // It will panic!
    // _a_biguint.lcm_assign_uint(_b_biguint);
    println!("---------------------------");
}



fn biguint_prime_gcd()
{
    println!("biguint_prime_gcd()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u32);

    // normal case
    let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
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
    let c_biguint = a_biguint.gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Panic Examples
    let _a_biguint = U256::zero();
    let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);

    let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = U256::zero();
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);

    let _a_biguint = U256::zero();
    let _b_biguint = U256::zero();
    // It will panic!
    // let c_biguint = _a_biguint.gcd(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_gcd_assign()
{
    println!("biguint_prime_gcd_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u64);

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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.gcd_assign(&b_biguint);
    println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic Examples
    let mut _a_biguint = U256::zero();
    let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // It will panic!
    // _a_biguint.gcd_assign(&_b_biguint);

    let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = U256::zero();
    // It will panic!
    // _a_biguint.gcd_assign(&_b_biguint);

    let mut _a_biguint = U256::zero();
    let _b_biguint = U256::zero();
    // It will panic!
    // _a_biguint.gcd_assign(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_extended_gcd()
{
    println!("biguint_prime_extended_gcd()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u128);

    // normal case
    let a_biguint = U256::from_string("240").unwrap();
    let b_biguint = U256::from_string("46").unwrap();
    let (c_biguint, x, y) = a_biguint.extended_gcd(&b_biguint);
    println!("The greatest common divisor of {} and {} is {} and x = -{}, y = {}.", a_biguint, b_biguint, c_biguint, U256::zero().wrapping_sub(&x), y);
    println!("overflow = {}, underflow = {}", x.is_overflow(), x.is_underflow());
    println!("---------------------------");
}

fn biguint_prime_lcm()
{
    println!("biguint_prime_lcm()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u16);

    // normal case
    let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
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
    let c_biguint = a_biguint.lcm(&b_biguint);
    println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // Panic Examples
    let _a_biguint = U256::zero();
    let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);

    let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = U256::zero();
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);

    let _a_biguint = U256::zero();
    let _b_biguint = U256::zero();
    // It will panic!
    // let c_biguint = _a_biguint.lcm(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_lcm_assign()
{
    println!("biguint_prime_lcm_assign()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u32);

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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    a_biguint.lcm_assign(&b_biguint);
    println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic Examples
    let mut _a_biguint = U256::zero();
    let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    // It will panic!
    // _a_biguint.lcm_assign(&_b_biguint);

    let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    let _b_biguint = U256::zero();
    // It will panic!
    // _a_biguint.lcm_assign(&_b_biguint);

    let mut _a_biguint = U256::zero();
    let _b_biguint = U256::zero();
    // It will panic!
    // _a_biguint.lcm_assign(&_b_biguint);
    println!("---------------------------");
}

fn biguint_prime_is_prime_using_miller_rabin()
{
    println!("biguint_prime_is_prime_using_miller_rabin()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_Prime;
    define_utypes_with!(u8);

    // prime numer case
    let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    assert_eq!(b, true);

    // composite number case
    let a_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    assert_eq!(b, false);
    println!("---------------------------");
}
