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
    biguint_quick_start_main();
    biguint_constructors_main();
    biguint_get_size_main();
    biguint_get_set_check_main();
    biguint_check_bits_main();
    biguint_comparison_uint_main();
    biguint_comparison_biguint_main();
    biguint_bit_operation_main();
    biguint_conversion_main();
    biguint_flag_manipulation_main();
}

fn biguint_quick_start_main()
{
    biguint_quick_start1();
    biguint_quick_start2();
}

fn biguint_quick_start1()
{
    println!("biguint_quick_start1");
    use std::str::FromStr;
    use cryptocol::number::*;

    type U1024 = BigUInt::<u128, 8>;

    let a_biguint = U1024::from([1_u128; 8]);
    println!("a_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nUndefined: {}\nDivided_by_Zero: {}\nLeft_Carry: {}\nRight_Carry: {}", a_biguint.get_number(), a_biguint.is_overflow(), a_biguint.is_underflow(), a_biguint.is_infinity(), a_biguint.is_undefined(), a_biguint.is_divided_by_zero(), a_biguint.is_left_carry(), a_biguint.is_right_carry());
    assert_eq!(*a_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    println!("a_biguint = {}", a_biguint);
    let txt = format!("{}", a_biguint);
    assert_eq!(txt, "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273");

    println!("a_biguint = {:#x}", a_biguint);
    let txt = format!("{:#x}", a_biguint);
    assert_eq!(txt, "0x100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    let b_biguint = U1024::from_string("528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273").unwrap();
    println!("b_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nUndefined: {}\nDivided_by_Zero: {}\nLeft_Carry: {}\nRight_Carry: {}", b_biguint.get_number(), b_biguint.is_overflow(), b_biguint.is_underflow(), b_biguint.is_infinity(), b_biguint.is_undefined(), b_biguint.is_divided_by_zero(), b_biguint.is_left_carry(), b_biguint.is_right_carry());
    assert_eq!(*b_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_left_carry(), false);
    assert_eq!(b_biguint.is_right_carry(), false);

    println!("b_biguint = {}", b_biguint);
    assert_eq!(b_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273");

    println!("b_biguint = {:X}", b_biguint);
    assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    let c_biguint = U1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();

    println!("c_biguint_biguint_biguint = {}", c_biguint);
    assert_eq!(c_biguint.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");

    let mut d_biguint = b_biguint.clone() + c_biguint.clone();
    println!("b_biguint + c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");

    d_biguint = b_biguint.clone() - c_biguint.clone();
    println!("b_biguint - c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");

    d_biguint = c_biguint.clone() - b_biguint.clone();
    println!("c_biguint_biguint - b_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");

    d_biguint = c_biguint.clone() * b_biguint.clone();
    println!("c_biguint_biguint * b_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");

    d_biguint = b_biguint.clone() / c_biguint.clone();
    println!("b_biguint / c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");

    d_biguint = b_biguint.clone() % c_biguint.clone();
    println!("b_biguint % c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");

    d_biguint = b_biguint.clone() + 5_u128;
    println!("b_biguint + 5 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");

    d_biguint = b_biguint.clone() - 1_u128;
    println!("b_biguint - 1 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");

    d_biguint = b_biguint.clone() * 42_u128;
    println!("b_biguint * 42 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");

    d_biguint = b_biguint.clone() / 5_u128;
    println!("b_biguint / 5 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");

    let e_uint = b_biguint.clone() % 5_u128;
    println!("b_uint % 5 = {}", e_uint);
    assert_eq!(e_uint, 3);
    println!("-------------------------------");
}

fn biguint_quick_start2()
{
    println!("biguint_quick_start2()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    
    define_utypes_with!(u128);

    let a_biguint = U1024::from([1; 8]);
    let b_biguint = U1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
    let c_biguint = UU128::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();

    println!("a_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", a_biguint.get_number(), a_biguint.is_overflow(), a_biguint.is_underflow(), a_biguint.is_infinity(), a_biguint.is_divided_by_zero());
    assert_eq!(*a_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    println!("a_biguint = {}", a_biguint.to_string_with_radix(16).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("b_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", b_biguint.get_number(), b_biguint.is_overflow(), b_biguint.is_underflow(), b_biguint.is_infinity(), b_biguint.is_divided_by_zero());
    assert_eq!(*b_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);

    println!("b_biguint = {}", b_biguint.to_string_with_radix(16).unwrap());
    assert_eq!(b_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

    println!("c_biguint_biguint = {}", c_biguint);
    assert_eq!(c_biguint.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");

    let mut d_biguint = c_biguint.wrapping_add(&b_biguint);
    println!("b_biguint + c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");

    d_biguint = b_biguint.wrapping_sub(&c_biguint);
    println!("b_biguint - c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");

    d_biguint = c_biguint.wrapping_sub(&b_biguint);
    println!("c_biguint - b_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");

    d_biguint = c_biguint.wrapping_mul(&b_biguint);
    println!("c_biguint * b_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");

    d_biguint = b_biguint.wrapping_div(&c_biguint);
    println!("b_biguint / c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");

    d_biguint = b_biguint.wrapping_rem(&c_biguint);
    println!("b_biguint % c_biguint = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");

    d_biguint = b_biguint.wrapping_add_uint(5_u128);
    println!("b_biguint + 5 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");

    d_biguint = b_biguint.wrapping_sub_uint(1_u128);
    println!("b_biguint - 1 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");

    d_biguint = b_biguint.wrapping_mul_uint(42_u128);
    println!("b_biguint * 42 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");

    d_biguint = b_biguint.wrapping_div_uint(5_u128);
    println!("b_biguint / 5 = {}", d_biguint);
    assert_eq!(d_biguint.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");

    let e_uint = b_biguint.wrapping_rem_uint(5_u128);
    println!("b_biguint % 5 = {}", e_uint);
    assert_eq!(e_uint, 3);
    println!("-------------------------------");
}

fn biguint_constructors_main()
{
    biguint_new();
    biguint_zero();
    biguint_one();
    biguint_max();
    biguint_submax();
    biguint_halfmax();
    biguint_from_uint();
    biguint_from_array();
    biguint_from_biguint();
    biguint_from_be();
    biguint_from_be_bytes();
    biguint_from_le();
    biguint_from_le_bytes();
    biguint_from_string();
    biguint_from_str_radix();
    biguint_generate_check_bits_();
    biguint_generate_check_bits();
}

fn biguint_new()
{
    println!("biguint_new");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let obj = U256::new();
    println!("obj = {}", obj);
    assert_eq!(obj.to_string(), "0");
    assert_eq!(obj.is_overflow(), false);
    assert_eq!(obj.is_underflow(), false);
    assert_eq!(obj.is_infinity(), false);
    assert_eq!(obj.is_divided_by_zero(), false);
    assert_eq!(obj.is_undefined(), false);
    assert_eq!(obj.is_left_carry(), false);
    assert_eq!(obj.is_right_carry(), false);
    println!("-------------------------------");
}

fn biguint_zero()
{
    println!("biguint_zero");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let zero = U256::zero();
    println!("zero = {}", zero);
    assert_eq!(zero.to_string(), "0");
    assert_eq!(zero.is_overflow(), false);
    assert_eq!(zero.is_underflow(), false);
    assert_eq!(zero.is_infinity(), false);
    assert_eq!(zero.is_divided_by_zero(), false);
    assert_eq!(zero.is_undefined(), false);
    assert_eq!(zero.is_left_carry(), false);
    assert_eq!(zero.is_right_carry(), false);
    println!("-------------------------------");
}

fn biguint_one()
{
    println!("biguint_one");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let one = U256::one();
    println!("one = {}", one);
    assert_eq!(one.to_string(), "1");
    assert_eq!(one.is_overflow(), false);
    assert_eq!(one.is_underflow(), false);
    assert_eq!(one.is_infinity(), false);
    assert_eq!(one.is_divided_by_zero(), false);
    assert_eq!(one.is_undefined(), false);
    assert_eq!(one.is_left_carry(), false);
    assert_eq!(one.is_right_carry(), false);
    println!("-------------------------------");
}

fn biguint_max()
{
    println!("biguint_max");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let maximum = U256::max();
    println!("maximum =\t{}", maximum);
    assert_eq!(maximum.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(maximum.is_overflow(), false);
    assert_eq!(maximum.is_underflow(), false);
    assert_eq!(maximum.is_infinity(), false);
    assert_eq!(maximum.is_divided_by_zero(), false);
    assert_eq!(maximum.is_undefined(), false);
    assert_eq!(maximum.is_left_carry(), false);
    assert_eq!(maximum.is_right_carry(), false);
    assert_eq!(maximum.wrapping_add_uint(1_u16), U256::zero());
    println!("---------------------------");
}

fn biguint_submax()
{
    println!("biguint_submax");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let half = U256::submax(128_u32);
    println!("half maximum = {0} = {0:#x}", half);
    println!("half maximum = \t{}", half.to_string_with_radix_and_stride(16, 4).unwrap());
    assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(half.is_overflow(), false);
    assert_eq!(half.is_underflow(), false);
    assert_eq!(half.is_infinity(), false);
    assert_eq!(half.is_divided_by_zero(), false);
    assert_eq!(half.is_undefined(), false);
    assert_eq!(half.is_left_carry(), false);
    assert_eq!(half.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_halfmax()
{
    println!("biguint_halfmax");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let half = U256::halfmax();
    println!("half maximum = {0} = {0:#x}", half);
    assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    println!("---------------------------");
}

fn biguint_from_uint()
{
    println!("biguint_from_uint");
    use cryptocol::define_utypes_with_u16;
    define_utypes_with_u16!();

    let a_from_u8 = U512::from_uint(123_u8);
    let b_from_u16 = U512::from_uint(12345_u16);
    let c_from_u32 = U512::from_uint(1234567890_u32);
    let d_from_u64 = U512::from_uint(12345678901234567890_u64);
    let e_from_u128 = U512::from_uint(123456789012345678901234567890123456789_u128);
    let f_from_usize = U512::from_uint(123_usize);

    println!("a_from_u8 = {}", a_from_u8);
    println!("b_from_u16 = {}", b_from_u16);
    println!("c_from_u32 = {}", c_from_u32);
    println!("d_from_u64 = {}", d_from_u64);
    println!("e_from_u128 = {}", e_from_u128);
    println!("f_from_usize = {}", f_from_usize);

    assert_eq!(a_from_u8.into_u8(), 123_u8);
    assert_eq!(a_from_u8.is_overflow(), false);
    assert_eq!(a_from_u8.is_underflow(), false);
    assert_eq!(a_from_u8.is_infinity(), false);
    assert_eq!(a_from_u8.is_divided_by_zero(), false);
    assert_eq!(a_from_u8.is_undefined(), false);
    assert_eq!(a_from_u8.is_left_carry(), false);
    assert_eq!(a_from_u8.is_right_carry(), false);

    assert_eq!(b_from_u16.into_u16(), 12345_u16);
    assert_eq!(b_from_u16.is_overflow(), false);
    assert_eq!(b_from_u16.is_underflow(), false);
    assert_eq!(b_from_u16.is_infinity(), false);
    assert_eq!(b_from_u16.is_divided_by_zero(), false);
    assert_eq!(b_from_u16.is_undefined(), false);
    assert_eq!(b_from_u16.is_left_carry(), false);
    assert_eq!(b_from_u16.is_right_carry(), false);
    
    assert_eq!(c_from_u32.into_u32(), 1234567890_u32);
    assert_eq!(c_from_u32.is_underflow(), false);
    assert_eq!(c_from_u32.is_infinity(), false);
    assert_eq!(c_from_u32.is_divided_by_zero(), false);
    assert_eq!(c_from_u32.is_undefined(), false);
    assert_eq!(c_from_u32.is_left_carry(), false);
    assert_eq!(c_from_u32.is_right_carry(), false);
    
    assert_eq!(d_from_u64.into_u64(), 12345678901234567890_u64);
    assert_eq!(d_from_u64.is_overflow(), false);
    assert_eq!(d_from_u64.is_underflow(), false);
    assert_eq!(d_from_u64.is_infinity(), false);
    assert_eq!(d_from_u64.is_divided_by_zero(), false);
    assert_eq!(d_from_u64.is_undefined(), false);
    assert_eq!(d_from_u64.is_left_carry(), false);
    assert_eq!(d_from_u64.is_right_carry(), false);
    
    assert_eq!(e_from_u128.into_u128(), 123456789012345678901234567890123456789_u128);
    assert_eq!(e_from_u128.is_overflow(), false);
    assert_eq!(e_from_u128.is_underflow(), false);
    assert_eq!(e_from_u128.is_infinity(), false);
    assert_eq!(e_from_u128.is_divided_by_zero(), false);
    assert_eq!(e_from_u128.is_undefined(), false);
    assert_eq!(e_from_u128.is_left_carry(), false);
    assert_eq!(e_from_u128.is_right_carry(), false);
    
    assert_eq!(f_from_usize.into_usize(), 123_usize);
    assert_eq!(f_from_usize.is_overflow(), false);
    assert_eq!(f_from_usize.is_underflow(), false);
    assert_eq!(f_from_usize.is_infinity(), false);
    assert_eq!(f_from_usize.is_divided_by_zero(), false);
    assert_eq!(f_from_usize.is_undefined(), false);
    assert_eq!(f_from_usize.is_left_carry(), false);
    assert_eq!(f_from_usize.is_right_carry(), false);

    let a_biguint = U256::from(123456789123456789123456789123456789_u128);
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789123456789123456789123456789");
    println!("---------------------------");
}

fn biguint_from_array()
{
    println!("biguint_from_array");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let big_num = U256::from_array([10_u8; 32]);
    println!("big_num = {:X}", big_num);
    assert_eq!(big_num.to_string_with_radix(16).unwrap(), "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A");
    assert_eq!(big_num.is_overflow(), false);
    assert_eq!(big_num.is_underflow(), false);
    assert_eq!(big_num.is_infinity(), false);
    assert_eq!(big_num.is_divided_by_zero(), false);
    assert_eq!(big_num.is_undefined(), false);
    assert_eq!(big_num.is_left_carry(), false);
    assert_eq!(big_num.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_from_biguint()
{
    println!("biguint_from_biguint");
    use std::str::FromStr;
    use cryptocol::number::*;

    let mut a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    a_u512_with_u8.set_overflow();
    a_u512_with_u8.set_underflow();
    a_u512_with_u8.set_infinity();
    a_u512_with_u8.set_divided_by_zero();
    a_u512_with_u8.set_undefined();
    a_u512_with_u8.set_left_carry();
    a_u512_with_u8.set_right_carry();
    assert_eq!(a_u512_with_u8.is_overflow(), true);
    assert_eq!(a_u512_with_u8.is_underflow(), true);
    assert_eq!(a_u512_with_u8.is_infinity(), true);
    assert_eq!(a_u512_with_u8.is_divided_by_zero(), true);
    assert_eq!(a_u512_with_u8.is_undefined(), true);
    assert_eq!(a_u512_with_u8.is_left_carry(), true);
    assert_eq!(a_u512_with_u8.is_right_carry(), true);

    // Example for the same length
    let b_u512_with_u8 = U512_with_u8::from_biguint(&a_u512_with_u8);
    println!("a_u512_with_u8 = {}", a_u512_with_u8);
    println!("b_u512_with_u8 = {}", b_u512_with_u8);
    assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    assert_eq!(b_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    assert_eq!(b_u512_with_u8.is_overflow(), false);
    assert_eq!(b_u512_with_u8.is_underflow(), false);
    assert_eq!(b_u512_with_u8.is_infinity(), false);
    assert_eq!(b_u512_with_u8.is_divided_by_zero(), false);
    assert_eq!(b_u512_with_u8.is_undefined(), false);
    assert_eq!(b_u512_with_u8.is_left_carry(), false);
    assert_eq!(b_u512_with_u8.is_right_carry(), false);

    // Example for the shorter length
    let b_u256_with_u8 = U256_with_u16::from_biguint(&a_u512_with_u8);
    println!("a_u512_with_u8 = {}", a_u512_with_u8);
    println!("b_u256_with_u8 = {}", b_u256_with_u8);
    assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    assert_eq!(b_u256_with_u8.to_string(), "98633800081229720571026865697976779988382011787853764870844783447569204535061");
    assert_eq!(b_u256_with_u8.is_overflow(), false);
    assert_eq!(b_u256_with_u8.is_underflow(), false);
    assert_eq!(b_u256_with_u8.is_infinity(), false);
    assert_eq!(b_u256_with_u8.is_divided_by_zero(), false);
    assert_eq!(b_u256_with_u8.is_undefined(), false);
    assert_eq!(b_u256_with_u8.is_left_carry(), false);
    assert_eq!(b_u256_with_u8.is_right_carry(), false);

    // Example for the longer length
    let b_u1024_with_u8 = U1024_with_u16::from_biguint(&a_u512_with_u8);
    println!("a_u512_with_u8 = {}", a_u512_with_u8);
    println!("b_u1024_with_u8 = {}", b_u1024_with_u8);
    assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    assert_eq!(b_u1024_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    assert_eq!(b_u1024_with_u8.is_overflow(), false);
    assert_eq!(b_u1024_with_u8.is_underflow(), false);
    assert_eq!(b_u1024_with_u8.is_infinity(), false);
    assert_eq!(b_u1024_with_u8.is_divided_by_zero(), false);
    assert_eq!(b_u1024_with_u8.is_undefined(), false);
    assert_eq!(b_u1024_with_u8.is_left_carry(), false);
    assert_eq!(b_u1024_with_u8.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_from_be()
{
    println!("biguint_from_be");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let be = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
                                0x1122, 0x3344, 0x5566, 0x7788,
                                0x9900, 0xaabb, 0xccdd, 0xeeff,
                                0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    let le = U256::from_be(be.clone());
    println!("be = {:#x}", be);
    println!("le = {:#x}", le);
    #[cfg(target_endian = "little")]
    {
        assert_eq!(be.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
        assert_eq!(le.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");        
    }
    #[cfg(target_endian = "big")]
    {
        assert_eq!(be.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
        assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");        
    }
    assert_eq!(le.is_overflow(), false);
    assert_eq!(le.is_underflow(), false);
    assert_eq!(le.is_infinity(), false);
    assert_eq!(le.is_divided_by_zero(), false);
    assert_eq!(le.is_undefined(), false);
    assert_eq!(le.is_left_carry(), false);
    assert_eq!(le.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_from_be_bytes()
{
    println!("biguint_from_be_bytes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let be_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
                    0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    let le = U256::from_be_bytes(be_array.clone());
    print!("be_array = ");
    for elem in be_array
        { print!("{:#8x} ", elem); }
    println!();
    println!("le = {:#x}", le);
    #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "78563412_EFCDAB90_44332211_88776655_BBAA0099_FFEEDDCC_4C3D2E1F_89706A5B");
    #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    assert_eq!(le.is_overflow(), false);
    assert_eq!(le.is_underflow(), false);
    assert_eq!(le.is_infinity(), false);
    assert_eq!(le.is_divided_by_zero(), false);
    assert_eq!(le.is_undefined(), false);
    assert_eq!(le.is_left_carry(), false);
    assert_eq!(le.is_right_carry(), false);
    println!("---------------------------");
}
fn biguint_from_le()
{
    println!("biguint_from_le");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let le1 = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
                    0x1122, 0x3344, 0x5566, 0x7788,
                    0x9900, 0xaabb, 0xccdd, 0xeeff,
                    0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    let le2 = U256::from_le(le1.clone());
    println!("le1 = {:#x}", le1);
    println!("le2 = {:#x}", le2);
    #[cfg(target_endian = "little")]
    {
        assert_eq!(le1.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
        assert_eq!(le2.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    }
    #[cfg(target_endian = "big")]
    {
        assert_eq!(le1.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
        assert_eq!(le2.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");
    }
    assert_eq!(le2.is_overflow(), false);
    assert_eq!(le2.is_underflow(), false);
    assert_eq!(le2.is_infinity(), false);
    assert_eq!(le2.is_divided_by_zero(), false);
    assert_eq!(le2.is_undefined(), false);
    assert_eq!(le2.is_left_carry(), false);
    assert_eq!(le2.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_from_le_bytes()
{
    println!("biguint_from_le_bytes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let le_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
                    0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    let le = U256::from_le_bytes(le_array.clone());
    print!("le_array = ");
    for elem in le_array
        { print!("{:#8x} ", elem); }
    println!();
    println!("le = {:#x}", le);
    #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "5B6A7089_1F2E3D4C_CCDDEEFF_9900AABB_55667788_11223344_90ABCDEF_12345678");
    #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    assert_eq!(le.is_overflow(), false);
    assert_eq!(le.is_underflow(), false);
    assert_eq!(le.is_infinity(), false);
    assert_eq!(le.is_divided_by_zero(), false);
    assert_eq!(le.is_undefined(), false);
    assert_eq!(le.is_left_carry(), false);
    assert_eq!(le.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_from_string()
{
    println!("biguint_from_string");
    use cryptocol::number::NumberErr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for correct case
    let a_correct = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    match a_correct
    {
        Ok(n) => {
            println!("a_correct = {}", n);
            assert_eq!(n.to_string(), "1234567890123456789012345678901234567890123456789012345678901234567890");
            assert_eq!(n.is_overflow(), false);
            assert_eq!(n.is_underflow(), false);
            assert_eq!(n.is_infinity(), false);
            assert_eq!(n.is_divided_by_zero(), false);
            assert_eq!(n.is_undefined(), false);
            assert_eq!(n.is_left_carry(), false);
            assert_eq!(n.is_right_carry(), false);
        },
        Err(e) => { println!("Failed: {}", e); },
    }

    // Example for NumberErr::NotAlphaNumeric case
    let b_contains_non_alphanumeric = U256::from_string("12345+67890");
    match b_contains_non_alphanumeric
    {
        Ok(n) =>  { println!("a_correct = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotAlphaNumeric);
        },
    }

    // Example for NumberErr::NotFitToRadix case
    let c_constains_not_fit_to_radix = U256::from_string("1234567890a");
    match c_constains_not_fit_to_radix
    {
        Ok(n) =>  { println!("c_constains_not_fit_to_radix = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotFitToRadix);
        },
    }

    // Example for NumberErr::TooBigNumber case
    let d_constains_too_big_number = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    match d_constains_too_big_number
    {
        Ok(n) =>  { println!("c_constains_too_big_number = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::TooBigNumber);
        },
    }

    // Example for NumberErr::NotAlphaNumeric and NumberErr::NotFitToRadix case
    let e_contains_non_alphanumeric_not_fit_to_radix = U256::from_string("F12345+67890");
    match e_contains_non_alphanumeric_not_fit_to_radix
    {
        Ok(n) =>  { println!("e_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotFitToRadix); // The first letter is 'F'.
        },
    }
    println!("---------------------------");
}

fn biguint_from_str_radix()
{
    println!("biguint_from_str_radix");
    use cryptocol::number::NumberErr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for correct case
    let a_correct = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    match a_correct
    {
        Ok(n) => {
            println!("a_correct = {}", n);
            assert_eq!(n.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0");
            assert_eq!(n.is_overflow(), false);
            assert_eq!(n.is_underflow(), false);
            assert_eq!(n.is_infinity(), false);
            assert_eq!(n.is_divided_by_zero(), false);
            assert_eq!(n.is_undefined(), false);
            assert_eq!(n.is_left_carry(), false);
            assert_eq!(n.is_right_carry(), false);
        },
        Err(e) => { println!("Failed: {}", e); },
    }

    // Example for NumberErr::OutOfValidRadixRange case
    let b_contains_out_of_valid_radix_range = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 63);
    match b_contains_out_of_valid_radix_range
    {
        Ok(n) =>  { println!("a_correct = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::OutOfValidRadixRange);
        },
    }

    // Example for NumberErr::NotAlphaNumeric case
    let c_contains_non_alphanumeric = U512::from_str_radix("1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0", 16);
    match c_contains_non_alphanumeric
    {
        Ok(n) =>  { println!("a_correct = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotAlphaNumeric);
        },
    }
    
    // Example for NumberErr::NotFitToRadix case
    let d_constains_not_fit_to_radix = U512::from_str_radix("1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG", 16);
    match d_constains_not_fit_to_radix
    {
        Ok(n) =>  { println!("d_constains_not_fit_to_radix = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotFitToRadix);
        },
    }

    // Example for NumberErr::TooBigNumber case
    let e_constains_too_big_number = U512::from_str_radix("1_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    match e_constains_too_big_number
    {
        Ok(n) =>  { println!("c_constains_too_big_number = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::TooBigNumber);
        },
    }

    // Example for NumberErr::NotAlphaNumeric, NumberErr::NotFitToRadix, and NumberErr::TooBigNumber case
    let f_contains_non_alphanumeric_not_fit_to_radix = U512::from_str_radix("1,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG", 16);
    match f_contains_non_alphanumeric_not_fit_to_radix
    {
        Ok(n) =>  { println!("f_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
        Err(e) => {
            println!("Failed: {}", e);
            assert_eq!(e, NumberErr::NotAlphaNumeric);
        },
    }
    println!("---------------------------");
}

fn biguint_generate_check_bits()
{
    println!("biguint_generate_check_bits");
    use cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let a_0 = U256::generate_check_bits(0).unwrap();
    println!("a_0 = {:#b}", a_0);
    assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    assert_eq!(a_0.is_overflow(), false);
    assert_eq!(a_0.is_underflow(), false);
    assert_eq!(a_0.is_infinity(), false);
    assert_eq!(a_0.is_divided_by_zero(), false);
    assert_eq!(a_0.is_undefined(), false);
    assert_eq!(a_0.is_left_carry(), false);
    assert_eq!(a_0.is_right_carry(), false);

    let a_12 = U256::generate_check_bits(12).unwrap();
    println!("a_12 = {:#b}", a_12);
    assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    assert_eq!(a_12.is_overflow(), false);
    assert_eq!(a_12.is_underflow(), false);
    assert_eq!(a_12.is_infinity(), false);
    assert_eq!(a_12.is_divided_by_zero(), false);
    assert_eq!(a_12.is_undefined(), false);
    assert_eq!(a_12.is_left_carry(), false);
    assert_eq!(a_12.is_right_carry(), false);

    let a_255 = U256::generate_check_bits(255).unwrap();
    println!("a_255 = {:#b}", a_255);
    assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    assert_eq!(a_255.is_overflow(), false);
    assert_eq!(a_255.is_underflow(), false);
    assert_eq!(a_255.is_infinity(), false);
    assert_eq!(a_255.is_divided_by_zero(), false);
    assert_eq!(a_255.is_undefined(), false);
    assert_eq!(a_255.is_left_carry(), false);
    assert_eq!(a_255.is_right_carry(), false);

    let a_256 = U256::generate_check_bits(256);
    match a_256
    {
        Some(n) => { println!("a_256 = {:#b}", n); },
        None => { assert_eq!(a_256, None); },
    }
    println!("---------------------------");
}

fn biguint_generate_check_bits_()
{
    println!("biguint_generate_check_bits_");
    use cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let a_0 = U256::generate_check_bits_(0);
    println!("a_0 = {}", a_0);
    assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    assert_eq!(a_0.is_overflow(), false);
    assert_eq!(a_0.is_underflow(), false);
    assert_eq!(a_0.is_infinity(), false);
    assert_eq!(a_0.is_divided_by_zero(), false);
    assert_eq!(a_0.is_undefined(), false);
    assert_eq!(a_0.is_left_carry(), false);
    assert_eq!(a_0.is_right_carry(), false);

    let a_12 = U256::generate_check_bits_(12);
    println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    assert_eq!(a_12.is_overflow(), false);
    assert_eq!(a_12.is_underflow(), false);
    assert_eq!(a_12.is_infinity(), false);
    assert_eq!(a_12.is_divided_by_zero(), false);
    assert_eq!(a_12.is_undefined(), false);
    assert_eq!(a_12.is_left_carry(), false);
    assert_eq!(a_12.is_right_carry(), false);

    let a_255 = U256::generate_check_bits_(255);
    println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    assert_eq!(a_255.is_overflow(), false);
    assert_eq!(a_255.is_underflow(), false);
    assert_eq!(a_255.is_infinity(), false);
    assert_eq!(a_255.is_divided_by_zero(), false);
    assert_eq!(a_255.is_undefined(), false);
    assert_eq!(a_255.is_left_carry(), false);
    assert_eq!(a_255.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_generate_check_bits_();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_generate_check_bits_()
{
    use cryptocol::define_utypes_with_u32;
    define_utypes_with_u32!();

    let _a_256 = U256::generate_check_bits_(256);
}

fn biguint_get_size_main()
{
    biguint_size_in_bytes();
    biguint_size_in_bits();
    biguint_length_in_bytes();
    biguint_length_in_bits();
}

fn biguint_size_in_bytes()
{
    println!("biguint_size_in_bytes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    println!("U256 is {}-byte integer.", U256::size_in_bytes());
    assert_eq!(U256::size_in_bytes(), 32);
    println!("---------------------------");
}

fn biguint_size_in_bits()
{
    println!("biguint_size_in_bits");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    println!("U256 is {}-bit integer.", U256::size_in_bits());
    assert_eq!(U256::size_in_bits(), 256);
    println!("---------------------------");
}

fn biguint_length_in_bytes()
{
    println!("biguint_length_in_bytes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("A16F", 16).unwrap();
    println!("a_biguint is {}-byte integer.", a_biguint.length_in_bytes());
    assert_eq!(a_biguint.length_in_bytes(), 32);
    println!("---------------------------");
}

fn biguint_length_in_bits()
{
    println!("biguint_length_in_bits");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("A16F", 16).unwrap();
    println!("a_biguint is {}-bit integer.", a_biguint.length_in_bits());
    assert_eq!(a_biguint.length_in_bits(), 256);
    println!("---------------------------");
}

fn biguint_get_set_check_main()
{
    biguint_turn_check_bits();
    biguint_is_bit_set();
    biguint_is_bit_set_();
    biguint_get_upper_portion();
    biguint_get_lower_portion();
    biguint_get_num();
    biguint_get_num_();
    biguint_set_num();
    biguint_set_num_();
    biguint_get_number();
    #[cfg(target_endian = "big")]   biguint_get_number_mut();
    biguint_set_number();
    // biguint_copy_within();
    biguint_set_zero();
    biguint_is_zero();
    biguint_set_one();
    biguint_is_one();
    biguint_is_zero_or_one();
    biguint_set_max();
    biguint_set_submax();
    biguint_set_halfmax();
    biguint_is_max();
    biguint_set_msb();
    biguint_set_lsb();
    biguint_set_uint();
    biguint_is_uint();
    biguint_is_odd();
    biguint_is_even();
    biguint_is_msb_set();
}

//=========
fn biguint_turn_check_bits()
{
    println!("biguint_turn_check_bits");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_string("256487951236974125896345564889974258").unwrap();
    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    a_biguint.turn_check_bits(102);
    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint, U256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());

    #[cfg(test)] // It will panic.
    biguint_should_panic_turn_check_bits();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_turn_check_bits()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut _a_biguint = U256::from_string("256487951236974125896345564889974258").unwrap();
    println!("_a_biguint = {}", _a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    _a_biguint.turn_check_bits(256);
}

fn biguint_is_bit_set()
{
    println!("biguint_is_bit_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    let res = a_biguint.is_bit_set(151);
    match res
    {
        Some(r) => {
            println!("The {}th bit is set: {}", 151, r);
            assert_eq!(a_biguint.is_bit_set_(151), true);
            assert_eq!(a_biguint.is_overflow(), false);
            assert_eq!(a_biguint.is_underflow(), false);
            assert_eq!(a_biguint.is_infinity(), false);
            assert_eq!(a_biguint.is_undefined(), false);
            assert_eq!(a_biguint.is_divided_by_zero(), false);
            assert_eq!(a_biguint.is_left_carry(), false);
            assert_eq!(a_biguint.is_right_carry(), false);
        },
        None => {
            println!("{}_U256 does not have the {}th bit.", a_biguint, 151);
        }
    }
    
    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    let res = a_biguint.is_bit_set(200);
    match res
    {
        Some(r) => {
            println!("The {}th bit is set: {}", 200, r);
            assert_eq!(a_biguint.is_bit_set_(200), false);
            assert_eq!(a_biguint.is_overflow(), false);
            assert_eq!(a_biguint.is_underflow(), false);
            assert_eq!(a_biguint.is_infinity(), false);
            assert_eq!(a_biguint.is_undefined(), false);
            assert_eq!(a_biguint.is_divided_by_zero(), false);
            assert_eq!(a_biguint.is_left_carry(), false);
            assert_eq!(a_biguint.is_right_carry(), false);
        },
        None => {
            println!("{}_U256 does not have the {}th bit.", a_biguint, 200);
        }
    }
    
    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    let res = a_biguint.is_bit_set(300);
    match res
    {
        Some(r) => { println!("The {}th bit is set: {}", 300, r); },
        None => {
            println!("{}_U256 does not have the {}th bit.", a_biguint, 300);
            assert_eq!(a_biguint.is_bit_set(300), None);
        }
    }
    println!("---------------------------");
}

fn biguint_is_bit_set_()
{
    println!("biguint_is_bit_set_");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    println!("The {}th bit is set: {}", 151, a_biguint.is_bit_set_(151));
    assert_eq!(a_biguint.is_bit_set_(151), true);
    println!("The {}th bit is set: {}", 200, a_biguint.is_bit_set_(200));
    assert_eq!(a_biguint.is_bit_set_(200), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_is_bit_set_();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_is_bit_set_()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let _a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    println!("The {}th bit is set: {}", 300, _a_biguint.is_bit_set_(300));
}

fn biguint_get_upper_portion()
{
    println!("biguint_get_upper_portion");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    let b = a_biguint.get_upper_portion(10);
    println!("The 10-bit upper portion of {}_U256 is {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101101001");
    println!("---------------------------");
}

fn biguint_get_lower_portion()
{
    println!("biguint_get_lower_portion");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912340").unwrap();
    let b = a_biguint.get_lower_portion(10);
    println!("The 10-bit lower portion of {}_U256 is {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101010100");
    println!("---------------------------");
}

fn biguint_get_num()
{
    println!("biguint_get_num");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    match a_biguint.get_num(3)
    {
        Some(num) => {
            println!("a_biguint.get_num(3).unwrap() = {}", num);
            assert_eq!(num, 30);
        },
        None => {
            println!("There is no third element.");
        },
    }

    let f = a_biguint.get_num(8);
    match f
    {
        Some(num) => {
            println!("a_biguint.get_num(3).unwrap() = {}", num);
        },
        None => {
            println!("There is no third element.");
            assert_eq!(f, None);
        },
    }
    println!("---------------------------");
}

fn biguint_get_num_()
{
    println!("biguint_get_num_");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    let b = a_biguint.get_num_(3);
    println!("a_biguint.get_num_(3) = {}", b);
    assert_eq!(b, 30);

    #[cfg(test)] // It will panic.
    biguint_should_panic_get_num_();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_get_num_()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _a_biguint = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    let _c = _a_biguint.get_num_(8);
}

fn biguint_set_num()
{
    println!("biguint_set_num");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from([0_u64, 10, 20, 30]);
    let mut num = a_biguint.get_num_(3);
    println!("a_biguint.get_num(3).unwrap() = {}", num);
    let b = a_biguint.set_num(3, 0);
    assert!(b);
    num = a_biguint.get_num_(3);
    println!("a_biguint.get_num(3).unwrap() = {}", num);
    assert_eq!(num, 0);

    let c = a_biguint.set_num(4, 0);
    if !c
        { println!("There is no fourth element."); }
    assert!(!c);
    println!("---------------------------");
}

fn biguint_set_num_()
{
    println!("biguint_set_num_");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from([10_u128, 20]);
    let mut num = a_biguint.get_num_(1);
    println!("a_biguint.get_num_(1) = {}", num);
    a_biguint.set_num_(1, 0);
    num = a_biguint.get_num_(1);
    println!("a_biguint.get_num_(1) = {}", num);
    assert_eq!(num, 0);

    #[cfg(test)] // It will panic.
    biguint_should_panic_set_num_();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_set_num_()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut _a_biguint = U256::from([10_u128, 20]);
    let _b = _a_biguint.set_num_(4, 0);
}

fn biguint_get_number()
{
    println!("biguint_get_number");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    if let Ok(a_biguint) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    {
        let arr = a_biguint.get_number();
        println!("arr = {:?}", arr);
        assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    }
    println!("---------------------------");
}

#[cfg(target_endian = "big")]
fn biguint_get_number_mut()
{
    println!("biguint_get_number_mut");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    if let Ok(a_biguint) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    {
        let arr = a_biguint.get_number_mut();
        println!("arr = {:?}", arr);
        assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    }
    println!("---------------------------");
}

fn biguint_set_number()
{
    println!("biguint_set_number");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::new();
    println!("arr = {:?}", a_biguint);
    let arr = [1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    a_biguint.set_number(&arr);
    println!("arr = {:?}", a_biguint);
    assert_eq!(a_biguint.get_number(), &arr);
    println!("---------------------------");
}

// fn biguint_copy_within()
// {
//     println!("biguint_copy_within");
//     use cryptocol::define_utypes_with;
//     define_utypes_with!(u16);
//     let mut a_biguint = U256::new();
//     a_biguint.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
//     println!("a_biguint = {:?}", a_biguint);
//     a_biguint.copy_within(3..10, 6);
//     println!("a_biguint = {:?}", a_biguint);
//     assert_eq!(a_biguint.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
//     println!("---------------------------");
// }

fn biguint_set_zero()
{
    println!("biguint_set_zero");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::new();
    a_biguint.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_zero();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, U256::zero());
    println!("---------------------------");
}

fn biguint_is_zero()
{
    println!("biguint_is_zero");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U1024::zero();
    let mut b_zero = a_biguint.is_zero();
    if b_zero
    {
        println!("a is Zero");
        assert_eq!(b_zero, true);
    }
    else
    {
        println!("a is Not Zero");
    }

    a_biguint.set_one();
    b_zero = a_biguint.is_zero();
    if b_zero
    {
        println!("a is Zero");
    }
    else
    {
        println!("a is Not Zero");
        assert_eq!(b_zero, false);
    }
    println!("---------------------------");
}

fn biguint_set_one()
{
    println!("biguint_set_one");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::new();
    a_biguint.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_one();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, U256::one());
    println!("---------------------------");
}

fn biguint_is_one()
{
    println!("biguint_is_one");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let mut a_biguint = U256::one();
    let mut b_one = a_biguint.is_one();
    if b_one
    {
        println!("a is One");
        assert_eq!(b_one, true);
    }
    else
    {
        println!("a is Not One");
    }

    a_biguint.set_max();
    b_one = a_biguint.is_one();
    if b_one
    {
        println!("a is One");
    }
    else
    {
        println!("a is Not One");
        assert_eq!(b_one, false);
    }
    println!("---------------------------");
}

fn biguint_is_zero_or_one()
{
    println!("biguint_is_zero_or_one");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::zero();
    println!("a_biguint = {}", a_biguint);
    let b_zero_or_one = a_biguint.is_zero_or_one();
    if b_zero_or_one
    {
        println!("a_biguint is One or Zero.");
        assert_eq!(b_zero_or_one, true);
    }
    else
    {
        println!("a_biguint is Neither One nor Zero.");
    }

    let a_biguint = U256::one();
    println!("a_biguint = {}", a_biguint);
    let b_zero_or_one = a_biguint.is_zero_or_one();
    if b_zero_or_one
    {
        println!("a_biguint is One or Zero.");
    }
    else
    {
        println!("a_biguint is Neither One nor Zero.");
        assert_eq!(b_zero_or_one, true);
    }

    let mut a_biguint = U256::one();
    a_biguint.wrapping_add_assign_uint(1_u8);
    println!("a_biguint = {}", a_biguint);
    let b_zero_or_one = a_biguint.is_zero_or_one();
    if b_zero_or_one
    {
        println!("a_biguint is One or Zero.");
    }
    else
    {
        println!("a_biguint is Neither One nor Zero.");
        assert_eq!(b_zero_or_one, false);
    }
    println!("---------------------------");
}

fn biguint_set_max()
{
    println!("biguint_set_max");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::new();
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_max();
    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    println!("---------------------------");
}

fn biguint_set_submax()
{
    println!("biguint_set_submax");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::new();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_submax(200_u32);
    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_halfmax()
{
    println!("biguint_set_halfmax");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::new();
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_halfmax();
    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    println!("---------------------------");
}

fn biguint_is_max()
{
    println!("biguint_is_max");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::max();
    println!("Is {} the 256-bit maximum? - {}", a_biguint, a_biguint.is_max());
    assert_eq!(a_biguint.is_max(), true);

    let a_biguint = U256::max().wrapping_sub_uint(1_u8);
    println!("Is {} the 256-bit maximum? - {}", a_biguint, a_biguint.is_max());
    assert_eq!(a_biguint.is_max(), false);
    println!("---------------------------");
}

fn biguint_set_msb()
{
    println!("biguint_set_msb");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::new();
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_msb();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    println!("---------------------------");
}

fn biguint_set_lsb()
{
    println!("biguint_set_lsb");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::new();
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_lsb();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    println!("---------------------------");
}

fn biguint_set_uint()
{
    println!("biguint_set_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U1024::new();
    println!("a_biguint = {}", a_biguint);
    a_biguint.set_uint(340282366920938463453374607431768211455_u128);
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "340282366920938463453374607431768211455");
    println!("---------------------------");
}

fn biguint_is_uint()
{
    println!("biguint_is_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U1024::one() + 50_u16;
    println!("Question: Is a 51?\nAnswer: {}", a_biguint.is_uint(51_u32));
    assert_eq!(a_biguint.is_uint(51_u16), true);
    assert_eq!(a_biguint.is_uint(50_u16), false);
    println!("---------------------------");
}

fn biguint_is_odd()
{
    println!("biguint_is_odd");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::new();
    a_biguint.set_uint(340282366920938463453374697431768211455_u128);
    if a_biguint.is_odd()
        { println!("{} is odd", a_biguint); }
    else
        { println!("{} is even", a_biguint); }
    assert_eq!(a_biguint.is_odd(), true);

    a_biguint <<= 1;
    if a_biguint.is_odd()
        { println!("{} is odd", a_biguint); }
    else
        { println!("{} is even", a_biguint); }
    assert_eq!(a_biguint.is_odd(), false);
    println!("---------------------------");
}

fn biguint_is_even()
{
    println!("biguint_is_even");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::new();
    a_biguint.set_uint(169743176821145534028236692093846345739_u128);
    if a_biguint.is_even()
        { println!("{} is even", a_biguint); }
    else
        { println!("{} is odd", a_biguint); }
    assert_eq!(a_biguint.is_even(), false);

    a_biguint <<= 1;
    if a_biguint.is_even()
        { println!("{} is even", a_biguint); }
    else
        { println!("{} is odd", a_biguint); }
    assert_eq!(a_biguint.is_even(), true);
    println!("---------------------------");
}

fn biguint_is_msb_set()
{
    println!("fn biguint_is_msb_set()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let a_biguint = U256::from_uint(169743176821145534028236692093846345739_u128);
    if a_biguint.is_msb_set()
        { println!("{} is greater than halfmax ({}).", a_biguint, U256::halfmax()); }
    else
        { println!("{} is less than or equal to halfmax ({}).", a_biguint, U256::halfmax()); }
    assert_eq!(a_biguint.is_msb_set(), false);

    let mut a_biguint = U256::from_uint(169743176821145534028236692093846345739_u128);
    a_biguint.set_msb();
    if a_biguint.is_msb_set()
        { println!("{} is greater than halfmax ({}).", a_biguint, U256::halfmax()); }
    else
        { println!("{} is less than or equal to halfmax ({}).", a_biguint, U256::halfmax()); }
    assert_eq!(a_biguint.is_msb_set(), true);
    println!("---------------------------");
}

fn biguint_check_bits_main()
{
    biguint_count_ones();
    biguint_count_zeros();
    biguint_leading_ones();
    biguint_leading_zeros();
    biguint_trailing_ones();
    biguint_trailing_zeros();
    biguint_leading_max_elements();
    biguint_leading_zero_elements();
    biguint_trailing_max_elements();
    biguint_trailing_zero_elements();
}

fn biguint_count_ones()
{
    println!("biguint_count_ones");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    println!("{} is {} in binary and has {} ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.count_ones());
    assert_eq!(a_biguint.count_ones(), 107);
    println!("---------------------------");
}

fn biguint_count_zeros()
{
    println!("biguint_count_zeros");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    println!("{} is {} in binary and has {} zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.count_zeros());
    assert_eq!(a_biguint.count_zeros(), 149);
    println!("---------------------------");
}

fn biguint_leading_ones()
{
    println!("biguint_leading_ones");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    println!("{} is {} in binary and has {} leading ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_ones());
    assert_eq!(a_biguint.leading_ones(), 2);
    println!("---------------------------");
}

fn biguint_leading_zeros()
{
    println!("biguint_leading_zeros");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = "100000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    println!("{} is {} in binary and has {} leading zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_zeros());
    assert_eq!(a_biguint.leading_zeros(), 10);
    println!("---------------------------");
}

fn biguint_trailing_ones()
{
    println!("biguint_trailing_ones");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();
    println!("{} is {} in binary and has {} trailing ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.trailing_ones());
    assert_eq!(a_biguint.trailing_ones(), 3);
    println!("---------------------------");
}

fn biguint_trailing_zeros()
{
    println!("biguint_trailing_zeros");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = "111111111111111111111111111111111111111111111111111111111111111111111111111111".parse::<U256>().unwrap();
    println!("{} is {} in binary and has {} trailing zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.trailing_zeros());
    assert_eq!(a_biguint.trailing_zeros(), 0);
    println!("---------------------------");
}

fn biguint_leading_max_elements()
{
    println!("biguint_leading_max_elements");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999_88888888", 16).unwrap();
    println!("{} is {} in hexadecimal and has {} leading max elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 2).unwrap(), a_biguint.leading_max_elements());
    assert_eq!(a_biguint.leading_max_elements(), 4);
    println!("---------------------------");
}

fn biguint_leading_zero_elements()
{
    println!("biguint_leading_zero_elements");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("00000000_FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999", 16).unwrap();
    println!("{} is {} in hexadecimal and has {} leading zero elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), a_biguint.leading_zero_elements());
    assert_eq!(a_biguint.leading_zero_elements(), 1);
    println!("---------------------------");
}

fn biguint_trailing_max_elements()
{
    println!("biguint_trailing_max_elements");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("88888888_99999999_AAAAAAAA_BBBBBBBB_CCCCCCCC_DDDDDDDD_EEEEEEEE_FFFFFFFF", 16).unwrap();
    println!("{} is {} in hexadecimal and has {} trailing max elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint.trailing_max_elements());
    assert_eq!(a_biguint.trailing_max_elements(), 2);
    println!("---------------------------");
}

fn biguint_trailing_zero_elements()
{
    println!("biguint_trailing_zero_elements");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_9999999_900000000", 16).unwrap();
    println!("{} is {} in hexadecimal and has {} trailing zero elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 2).unwrap(), a_biguint.trailing_zero_elements());
    assert_eq!(a_biguint.trailing_zero_elements(), 4);
    println!("---------------------------");
}

fn biguint_comparison_uint_main()
{
    biguint_partial_cmp_uint();
    biguint_lt_uint();
    biguint_gt_uint();
    biguint_le_uint();
    biguint_ge_uint();
    biguint_eq_uint();
}

fn biguint_partial_cmp_uint()
{
    println!("biguint_partial_cmp_uint");
    use std::cmp::Ordering;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let res = UU32::from_uint(100_u8).partial_cmp_uint(90_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 90"); }
        Ordering::Less => { println!("100 < 90"); }
        Ordering::Equal => { println!("100 = 90"); }
    }
    assert_eq!(res, Ordering::Greater);

    let res = UU32::from_uint(100_u8).partial_cmp_uint(110_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 110"); }
        Ordering::Less => { println!("100 < 110"); }
        Ordering::Equal => { println!("100 = 110"); }
    }
    assert_eq!(res, Ordering::Less);

    let res = UU32::from_uint(100_u8).partial_cmp_uint(100_u128).unwrap();
    match res
    {
        Ordering::Greater => { println!("100 > 100"); }
        Ordering::Less => { println!("100 < 100"); }
        Ordering::Equal => { println!("100 = 100"); }
    }
    assert_eq!(res, Ordering::Equal);
    println!("---------------------------");
}

fn biguint_lt_uint()
{
    println!("biguint_lt_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint.lt_uint(b_uint);
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint.lt_uint(b_uint);
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint.lt_uint(b_uint);
    if res
        { println!("{} < {}", a_biguint, b_uint); }
    else
        { println!("{} >= {}", a_biguint, b_uint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_gt_uint()
{
    println!("biguint_gt_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint.gt_uint(b_uint);
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint.gt_uint(b_uint);
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint.gt_uint(b_uint);
    if res
        { println!("{} > {}", a_biguint, b_uint); }
    else
        { println!("{} <= {}", a_biguint, b_uint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_le_uint()
{
    println!("biguint_le_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint.le_uint(b_uint);
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint.le_uint(b_uint);
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint.le_uint(b_uint);
    if res
        { println!("{} <= {}", a_biguint, b_uint); }
    else
        { println!("{} > {}", a_biguint, b_uint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_ge_uint()
{
    println!("biguint_ge_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = UU32::from_uint(200_u8);
    let b_uint = 100_u8;
    let res = a_biguint.ge_uint(b_uint);
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint.ge_uint(b_uint);
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, false);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint.ge_uint(b_uint);
    if res
        { println!("{} >= {}", a_biguint, b_uint); }
    else
        { println!("{} < {}", a_biguint, b_uint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_eq_uint()
{
    println!("biguint_eq_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 100_u8;
    let res = a_biguint.eq_uint(b_uint);
    if res
        { println!("{} == {}", a_biguint, b_uint); }
    else
        { println!("{} != {}", a_biguint, b_uint); }
    assert_eq!(res, true);

    let a_biguint = UU32::from_uint(100_u8);
    let b_uint = 200_u8;
    let res = a_biguint.eq_uint(b_uint);
    if res
        { println!("{} == {}", a_biguint, b_uint); }
    else
        { println!("{} != {}", a_biguint, b_uint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_comparison_biguint_main()
{
    biguint_eq();
    biguint_partial_cmp();
    biguint_lt();
    biguint_gt();
    biguint_le();
    biguint_ge();
    biguint_eq();
}

fn biguint_partial_cmp()
{
    println!("biguint_partial_cmp");
    use std::cmp::Ordering;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num_str2 = "60000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num_str3 = "80000000000000000000000000000000000000000000000000000000000000000000000000000";
    let num1 = num_str1.parse::<UU32>().unwrap();
    let num2 = num_str2.parse::<UU32>().unwrap();
    let num3 = num_str3.parse::<UU32>().unwrap();

    let res = num1.partial_cmp(&num2).unwrap();
    match res
    {
        Ordering::Greater => { println!("{} > {}", num1, num2); }
        Ordering::Less => { println!("{} < {}", num1, num2); }
        Ordering::Equal => { println!("{} = {}", num1, num2); }
    }
    assert_eq!(res, Ordering::Greater);

    let res = num1.partial_cmp(&num3).unwrap();
    match res
    {
        Ordering::Greater => { println!("{} > {}", num1, num3); }
        Ordering::Less => { println!("{} < {}", num1, num3); }
        Ordering::Equal => { println!("{} = {}", num1, num3); }
    }
    assert_eq!(res, Ordering::Less);

    let res = num1.partial_cmp(&num1).unwrap();
    match res
    {
        Ordering::Greater => { println!("{0} > {0}", num1); }
        Ordering::Less => { println!("{0} < {0}", num1); }
        Ordering::Equal => { println!("{0} = {0}", num1); }
    }
    assert_eq!(res, Ordering::Equal);
    println!("---------------------------");
}

fn biguint_lt()
{
    println!("biguint_lt");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint.lt(&b_biguint);
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.lt(&b_biguint);
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.lt(&b_biguint);
    if res
        { println!("{} < {}", a_biguint, b_biguint); }
    else
        { println!("{} >= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_gt()
{
    println!("biguint_gt");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint.gt(&b_biguint);
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.gt(&b_biguint);
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.gt(&b_biguint);
    if res
        { println!("{} > {}", a_biguint, b_biguint); }
    else
        { println!("{} <= {}", a_biguint, b_biguint); }
    assert_eq!(res, false);
    println!("---------------------------");
}

fn biguint_le()
{
    println!("biguint_le_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint.le(&b_biguint);
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.le(&b_biguint);
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.le(&b_biguint);
    if res
        { println!("{} <= {}", a_biguint, b_biguint); }
    else
        { println!("{} > {}", a_biguint, b_biguint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_ge()
{
    println!("biguint_ge");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint.ge(&b_biguint);
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_uint(100_u8);
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.ge(&b_biguint);
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, false);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.ge(&b_biguint);
    if res
        { println!("{} >= {}", a_biguint, b_biguint); }
    else
        { println!("{} < {}", a_biguint, b_biguint); }
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_eq()
{
    println!("biguint_eq");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_string(num_str).unwrap();
    let res = a_biguint.eq(&b_biguint);
    if res
        { println!("{} = {}", a_biguint, b_biguint); }
    else
        { println!("{} != {}", a_biguint, b_biguint); }
    assert_eq!(res, true);

    let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    let a_biguint = UU32::from_string(num_str).unwrap();
    let b_biguint = UU32::from_uint(100_u8);
    let res = a_biguint.eq(&b_biguint);
    if res
        { println!("{} = {}", a_biguint, b_biguint); }
    else
        { println!("{} != {}", a_biguint, b_biguint); }
    assert_eq!(res, false);
    println!("---------------------------");
}



fn biguint_bit_operation_main()
{
    biguint_shift_left();
    biguint_shift_left_assign();
    biguint_shift_right();
    biguint_shift_right_assign();
    biguint_rotate_left();
    biguint_rotate_left_assign();
    biguint_rotate_right();
    biguint_rotate_right_assign();
    biguint_and();
    biguint_and_assign();
    biguint_or();
    biguint_or_assign();
    biguint_xor();
    biguint_xor_assign();
    biguint_flip();
    biguint_flip_assign();
    biguint_reverse_bits();
    biguint_reverse_bits_assign();
    biguint_swap_bytes();
    biguint_swap_bytes_assign();
}

fn biguint_shift_left()
{
    println!("biguint_shift_left()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.shift_left(n);
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
    let res = a_biguint.shift_left(n);
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
    let res = a_biguint.shift_left(n);
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
    let n = 256_u16;
    let res = a_biguint.shift_left(n);
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
    let n = 512_u16;
    let res = a_biguint.shift_left(n);
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_shift_left_assign()
{
    println!("biguint_shift_left_assign()");
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
    a_biguint.shift_left_assign(n);
    println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 4_u8;
    a_biguint.shift_left_assign(n);
    println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 128_u8;
    a_biguint.shift_left_assign(n);
    println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 256_u16;
    a_biguint.shift_left_assign(n);
    println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 512_u16;
    a_biguint.shift_left_assign(n);
    println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_shift_right()
{
    println!("biguint_shift_right()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.shift_right(n);
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
    let res = a_biguint.shift_right(n);
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
    let res = a_biguint.shift_right(n);
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
    let n = 256_u16;
    let res = a_biguint.shift_right(n);
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
    let n = 512_u16;
    let res = a_biguint.shift_right(n);
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);
    println!("---------------------------");
}

fn biguint_shift_right_assign()
{
    println!("biguint_shift_right_assign()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

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
    a_biguint.shift_right_assign(n);
    println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 4_u8;
    a_biguint.shift_right_assign(n);
    println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 128_u8;
    a_biguint.shift_right_assign(n);
    println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 256_u16;
    a_biguint.shift_right_assign(n);
    println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 512_u16;
    a_biguint.shift_right_assign(n);
    println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_rotate_left()
{
    println!("biguint_rotate_left()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.rotate_left(n);
    println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.rotate_left(n);
    println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000_00001010");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.rotate_left(n);
    println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 256_u16;
    let res = a_biguint.rotate_left(n);
    println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 512_u16;
    let res = a_biguint.rotate_left(n);
    println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_rotate_left_assign()
{
    println!("biguint_rotate_left_assign()");
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

    let n = 3_u8;
    a_biguint.rotate_left_assign(n);
    println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
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

    let n = 4_u8;
    a_biguint.rotate_left_assign(n);
    println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000_00001010");
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

    let n = 128_u8;
    a_biguint.rotate_left_assign(n);
    println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
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

    let n = 256_u16;
    a_biguint.rotate_left_assign(n);
    println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 512_u16;
    a_biguint.rotate_left_assign(n);
    println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_rotate_right()
{
    println!("biguint_rotate_right()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.rotate_right(n);
    println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.rotate_right(n);
    println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010_10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.rotate_right(n);
    println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 256_u16;
    let res = a_biguint.rotate_right(n);
    println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let n = 512_u16;
    let res = a_biguint.rotate_right(n);
    println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_rotate_right_assign()
{
    println!("biguint_rotate_right_assign()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let n = 3_u8;
    a_biguint.rotate_right_assign(n);
    println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
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

    let n = 4_u8;
    a_biguint.rotate_right_assign(n);
    println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010_10101100_11001111_00001111_11110000_00001111_11111111_11110000_00000000_00001111_11111111_11111111_11110000_00000000_00000000_00001111_11111111_11111111_11111111_11110000_00000000_00000000_00000000_00001011_00111000_11110000_11111000_00111111_00000011_11111000_00001111_11110000");
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

    let n = 128_u8;
    a_biguint.rotate_right_assign(n);
    println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111");
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

    let n = 256_u16;
    a_biguint.rotate_right_assign(n);
    println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

    let n = 512_u16;
    a_biguint.rotate_right_assign(n);
    println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_and()
{
    println!("biguint_and()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.and(&b_biguint);
    
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
    let c_biguint = a_biguint.and(&b_biguint);
    
    println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::zero();
    let c_biguint = a_biguint.and(&b_biguint);
    
    println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string(), "0");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_and_assign()
{
    println!("biguint_and_assign()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

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
    a_biguint.and_assign(&b_biguint);
    println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.and_assign(&b_biguint);
    println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.and_assign(&b_biguint);
    println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_or()
{
    println!("biguint_bitor()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.or(&b_biguint);
    
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
    let c_biguint = a_biguint.or(&b_biguint);
    
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
    let c_biguint = a_biguint.or(&b_biguint);
    
    println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_or_assign()
{
    println!("biguint_or_assign()");
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
    a_biguint.or_assign(&b_biguint);
    println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.or_assign(&b_biguint);
    println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.or_assign(&b_biguint);
    println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_xor()
{
    println!("biguint_xor()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    let c_biguint = a_biguint.xor(&b_biguint);
    
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
    let c_biguint = a_biguint.xor(&b_biguint);
    
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
    let c_biguint = a_biguint.xor(&b_biguint);
    
    println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_xor_assign()
{
    println!("biguint_xor_assign()");
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
    a_biguint.xor_assign(&b_biguint);
    println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.xor_assign(&b_biguint);
    println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    a_biguint.xor_assign(&b_biguint);
    println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_flip()
{
    println!("biguint_flip()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.flip();
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
    let res = a_biguint.flip();
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
    let res = a_biguint.flip();
    println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_flip_assign()
{
    println!("biguint_flip_assign()");
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

    a_biguint.flip_assign();
    println!("After a_biguint.flip_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    // assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.flip_assign();
    println!("After a_biguint.flip_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.flip_assign();
    println!("After a_biguint.flip_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reverse_bits()
{
    println!("biguint_reverse_bits()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.reverse_bits();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::max();
    let res = a_biguint.reverse_bits();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.reverse_bits();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_reverse_bits_assign()
{
    println!("biguint_reverse_bits_assign()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reverse_bits_assign();
    println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reverse_bits_assign();
    println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reverse_bits_assign();
    println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_swap_bytes()
{
    println!("biguint_swap_bytes()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.swap_bytes();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::max();
    let res = a_biguint.swap_bytes();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.swap_bytes();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_swap_bytes_assign()
{
    println!("biguint_swap_bytes_assign()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.swap_bytes_assign();
    println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.swap_bytes_assign();
    println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.swap_bytes_assign();
    println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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


fn biguint_conversion_main()
{
    biguint_into_biguint();
    biguint_into_uint();
    biguint_into_u128();
    biguint_into_u64();
    biguint_into_u32();
    biguint_into_u16();
    biguint_into_u8();
    biguint_into_usize();
    biguint_to_be();
    biguint_to_be_assign();
    biguint_to_be_bytes();
    biguint_to_le();
    biguint_to_le_assign();
    biguint_to_le_bytes();
    biguint_to_string_with_radix_and_stride_and_delimiter();
    biguint_to_string_with_radix_and_stride();
    biguint_to_string_with_radix();
}

fn biguint_into_biguint()
{
    println!("biguint_into_biguint()");
    use cryptocol::number::BigUInt;
    use cryptocol::number::U256_with_u128;
    use cryptocol::number::U256_with_u8;
    use cryptocol::number::U512_with_u8;
    use cryptocol::number::U512_with_u128;
    use std::fmt::Write;

    let mut a_biguint = U256_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    a_biguint.set_overflow();
    a_biguint.set_underflow();
    a_biguint.set_undefined();
    a_biguint.set_infinity();
    a_biguint.set_divided_by_zero();
    a_biguint.set_left_carry();
    a_biguint.set_right_carry();

    let b_biguint: BigUInt<u16, 32> = a_biguint.into_biguint();
    println!("a_biguint = {0} = {0:?}", a_biguint);
    println!("b_biguint = {0} = {0:?}", b_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), true);

    assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_left_carry(), false);
    assert_eq!(b_biguint.is_right_carry(), false);
    
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", a_biguint)
    {
        Ok(_) =>    { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 127 }"); },
        Err(_) =>   { panic!("Error"); },
    }
    let mut b_txt = String::new();
    match write!(&mut b_txt, "{:?}", b_biguint)
    {
        Ok(_) => { assert_eq!(b_txt, "BigUInt { number: [65280, 16256, 33776, 36623, 179, 0, 65280, 65535, 255, 0, 65535, 255, 65280, 255, 61695, 43724, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], flag: 0 }"); },
        Err(_) =>   { panic!("Error"); },
    }

    let mut a_biguint = U256_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    a_biguint.set_overflow();
    a_biguint.set_underflow();
    a_biguint.set_undefined();
    a_biguint.set_infinity();
    a_biguint.set_divided_by_zero();
    a_biguint.set_left_carry();
    a_biguint.set_right_carry();

    let b_biguint: U512_with_u8 = a_biguint.into_biguint();
    println!("a_biguint = {0} = {0:?}", a_biguint);
    println!("b_biguint = {0} = {0:?}", b_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), true);

    assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_left_carry(), false);
    assert_eq!(b_biguint.is_right_carry(), false);

    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", a_biguint)
    {
        Ok(_) =>    { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 127 }"); },
        Err(_) =>   { panic!("Error"); },
    }
    let mut b_txt = String::new();
    match write!(&mut b_txt, "{:?}", b_biguint)
    {
        Ok(_) =>    { assert_eq!(b_txt, "BigUInt { number: [0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], flag: 0 }"); },
        Err(_) =>   { panic!("Error"); },
    }

    let mut a_biguint = U512_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    a_biguint.set_overflow();
    a_biguint.set_underflow();
    a_biguint.set_undefined();
    a_biguint.set_infinity();
    a_biguint.set_divided_by_zero();
    a_biguint.set_left_carry();
    a_biguint.set_right_carry();

    let b_biguint: U256_with_u8 = a_biguint.into_biguint();
    println!("a_biguint = {0} = {0:?}", a_biguint);
    println!("b_biguint = {0} = {0:?}", b_biguint);
    assert_eq!(a_biguint.to_string(), "8945550780017187584626056870222733452660064686360582980627279346698888314793843532145493214749705164311564838731068213948692682076110455767663905463140096");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), true);

    assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_left_carry(), false);
    assert_eq!(b_biguint.is_right_carry(), false);

    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", a_biguint)
    {
        Ok(_) => { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279, 340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 127 }"); },
        Err(_) =>   { panic!("Error"); },
    }
    let mut b_txt = String::new();
    match write!(&mut b_txt, "{:?}", b_biguint)
    {
        Ok(_) => { assert_eq!(b_txt, "BigUInt { number: [0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170], flag: 0 }"); },
        Err(_) =>   { panic!("Error"); },
    }
    println!("---------------------------");
}

fn biguint_into_uint()
{
    println!("biguint_into_uint()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_uint(16384545419507531775_u64);
    let b_u128: u128 = a_biguint.into_uint();
    let b_u64: u64 = a_biguint.into_uint();
    let b_u32: u32 = a_biguint.into_uint();
    let b_u16: u16 = a_biguint.into_uint();
    let b_u8: u8 = a_biguint.into_uint();
    println!("u128 of {} = {}", a_biguint, b_u128);
    println!("u64 of {} = {}", a_biguint, b_u64);
    println!("u32 of {} = {}", a_biguint, b_u32);
    println!("u16 of {} = {}", a_biguint, b_u16);
    println!("u8 of {} = {}", a_biguint, b_u8);
    assert_eq!(b_u128, 16384545419507531775_u128);
    assert_eq!(b_u64, 16384545419507531775_u64);
    assert_eq!(b_u32, 4294967295_u32);
    assert_eq!(b_u16, 65535_u16);
    assert_eq!(b_u8, 255_u8);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u128: u128 = a_biguint.into_uint();
    let b_u64: u64 = a_biguint.into_uint();
    let b_u32: u32 = a_biguint.into_uint();
    let b_u16: u16 = a_biguint.into_uint();
    let b_u8: u8 = a_biguint.into_uint();
    println!("u128 of {} = {}", a_biguint, b_u128);
    println!("u64 of {} = {}", a_biguint, b_u64);
    println!("u32 of {} = {}", a_biguint, b_u32);
    println!("u16 of {} = {}", a_biguint, b_u16);
    println!("u8 of {} = {}", a_biguint, b_u8);
    assert_eq!(b_u128, 340282346638528863123979975818481827584_u128);
    assert_eq!(b_u64, 10308603139955162880_u64);
    assert_eq!(b_u32, 1065418496_u32);
    assert_eq!(b_u16, 65280_u16);
    assert_eq!(b_u8, 0_u8);
    println!("---------------------------");
}

fn biguint_into_u128()
{
    println!("biguint_into_u128()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_uint(16384545419507531775_u64);
    let b_u128 = a_biguint.into_u128();
    println!("u128 of {} = {}", a_biguint, b_u128);
    assert_eq!(b_u128, 16384545419507531775_u128);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u128: u128 = a_biguint.into_u128();
    println!("u128 of {} = {}", a_biguint, b_u128);
    assert_eq!(b_u128, 340282346638528863123979975818481827584_u128);
    println!("---------------------------");
}

fn biguint_into_u64()
{
    println!("biguint_into_u64()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_uint(16384545419507531775_u64);
    let b_u64: u64 = a_biguint.into_u64();
    println!("u64 of {} = {}", a_biguint, b_u64);
    assert_eq!(b_u64, 16384545419507531775_u64);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u64: u64 = a_biguint.into_u64();
    println!("u64 of {} = {}", a_biguint, b_u64);
    assert_eq!(b_u64, 10308603139955162880_u64);
    println!("---------------------------");
}

fn biguint_into_u32()
{
    println!("biguint_into_u32()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_uint(163_u8);
    let b_u32 = a_biguint.into_u32();
    println!("u32 of {} = {}", a_biguint, b_u32);
    assert_eq!(b_u32, 163_u32);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u32 = a_biguint.into_u32();
    println!("u32 of {} = {}", a_biguint, b_u32);
    assert_eq!(b_u32, 1065418496_u32);
    println!("---------------------------");
}

fn biguint_into_u16()
{
    println!("biguint_into_u16()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_uint(163_u8);
    let b_u16 = a_biguint.into_u16();
    println!("u16 of {} = {}", a_biguint, b_u16);
    assert_eq!(b_u16, 163_u16);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u16 = a_biguint.into_u16();
    println!("u16 of {} = {}", a_biguint, b_u16);
    assert_eq!(b_u16, 65280_u16);
    println!("---------------------------");
}

fn biguint_into_u8()
{
    println!("biguint_into_u8()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_uint(163_u8);
    let b_u8: u8 = a_biguint.into_u8();
    println!("u8 of {} = {}", a_biguint, b_u8);
    assert_eq!(b_u8, 163_u8);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_u8: u8 = a_biguint.into_u8();
    println!("u8 of {} = {}", a_biguint, b_u8);
    assert_eq!(b_u8, 0_u8);
    println!("---------------------------");
}

fn biguint_into_usize()
{
    println!("biguint_into_usize()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_uint(65280_u16);
    let b_usize = a_biguint.into_usize();
    println!("usize of {} = {}", a_biguint, b_usize);
    assert_eq!(b_usize, 65280_usize);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    let b_usize = a_biguint.into_usize();
    println!("usize of {} = {}", a_biguint, b_usize);
    #[cfg(target_pointer_width = "64")] assert_eq!(b_usize, 10308603139955162880_usize);
    #[cfg(target_pointer_width = "32")] assert_eq!(b_usize, 1065418496_usize);
    #[cfg(target_pointer_width = "16")] assert_eq!(b_usize, 65280_usize);
    println!("---------------------------");
}

fn biguint_to_be()
{
    println!("biguint_to_be()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.to_be();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::max();
    let res = a_biguint.to_be();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.to_be();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_to_be_assign()
{
    println!("biguint_to_be_assign()");
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

    a_biguint.to_be_assign();
    println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.to_be_assign();
    println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.to_be_assign();
    println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    println!("---------------------------");
}

fn biguint_to_be_bytes()
{
    println!("biguint_to_be_bytes()");
    use std::fmt::Write;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.to_be_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => {
                #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
                #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
            },
        Err(_) => { panic!("Error"); },
    }

    let a_biguint = U256::max();
    let res = a_biguint.to_be_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => { assert_eq!(a_txt, "[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]"); },
        Err(_) => { panic!("Error"); },
    }

    let a_biguint = U256::zero();
    let res = a_biguint.to_be_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => { assert_eq!(a_txt, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"); },
        Err(_) => { panic!("Error"); },
    }
    println!("---------------------------");
}

fn biguint_to_le()
{
    println!("biguint_to_le()");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.to_le();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::max();
    let res = a_biguint.to_le();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.to_le();
    println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_to_le_assign()
{
    println!("biguint_to_le_assign()");
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

    a_biguint.to_le_assign();
    println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.to_le_assign();
    println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.to_le_assign();
    println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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

fn biguint_to_le_bytes()
{
    println!("biguint_to_le_bytes()");
    use std::fmt::Write;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    let res = a_biguint.to_le_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => {
                #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
                #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
            },
        Err(_) => { panic!("Error"); },
    }

    let a_biguint = U256::max();
    let res = a_biguint.to_le_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => { assert_eq!(a_txt, "[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]"); },
        Err(_) => { panic!("Error"); },
    }

    let a_biguint = U256::zero();
    let res = a_biguint.to_le_bytes();
    println!("{:?} => {:?}", a_biguint, res);
    let mut a_txt = String::new();
    match write!(&mut a_txt, "{:?}", res)
    {
        Ok(_) => { assert_eq!(a_txt, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"); },
        Err(_) => { panic!("Error"); },
    }
    println!("---------------------------");
}

fn biguint_to_string_with_radix_and_stride_and_delimiter()
{
    println!("biguint_to_string_with_radix_and_stride_and_delimiter()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::NumberErr;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap(), "77,255,284,354,385,016,970,177,264,758,879,158,019,392,010,587,479,561,699,232,008,238,232,688,983,808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(16, 4, "-").unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(16, 4, "-").unwrap(), "AACC-F0FF-00FF-FF00-00FF-FFFF-0000-00FF-FFFF-FF00-0000-00B3-8F0F-83F0-3F80-FF00");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(2, 8, "_").unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(2, 8, "_").unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(36, 4, ":").unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(36, 4, ":").unwrap(), "49:93ID:4SD9:M4DT:2QO9:EF7V:ZKGD:LH3S:Y0SO:W4CH:RKE5:CQA4:0MPS");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(62, 4, "~").unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(62, 4, ":").unwrap(), "eV7:xhnH:Dmgs:yLnq:m5P9:ZaJf:dOP0:7xlq:S2Da:BiV2:F7dg");

    match a_biguint.to_string_with_radix_and_stride_and_delimiter(1, 4, "$")
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }

    match a_biguint.to_string_with_radix_and_stride_and_delimiter(63, 4, "~")
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }
    println!("---------------------------");
}

fn biguint_to_string_with_radix_and_stride()
{
    println!("biguint_to_string_with_radix_and_stride()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::NumberErr;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(10, 3).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 3).unwrap(), "77_255_284_354_385_016_970_177_264_758_879_158_019_392_010_587_479_561_699_232_008_238_232_688_983_808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 4).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 4).unwrap(), "AACC_F0FF_00FF_FF00_00FF_FFFF_0000_00FF_FFFF_FF00_0000_00B3_8F0F_83F0_3F80_FF00");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(36, 6).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(36, 6).unwrap(), "49_93ID4S_D9M4DT_2QO9EF_7VZKGD_LH3SY0_SOW4CH_RKE5CQ_A40MPS");

    println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(62, 5).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(62, 5).unwrap(), "eV7_xhnHD_mgsyL_nqm5P_9ZaJf_dOP07_xlqS2_DaBiV_2F7dg");
  
    match a_biguint.to_string_with_radix_and_stride(1, 4)
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }

    match a_biguint.to_string_with_radix_and_stride(63, 5)
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }
    println!("---------------------------");
}

fn biguint_to_string_with_radix()
{
    println!("biguint_to_string_with_radix()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::NumberErr;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix(10).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(10).unwrap(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");

    println!("a_biguint = {}", a_biguint.to_string_with_radix(16).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "AACCF0FF00FFFF0000FFFFFF000000FFFFFFFF00000000B38F0F83F03F80FF00");

    println!("a_biguint = {}", a_biguint.to_string_with_radix(2).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(2).unwrap(), "1010101011001100111100001111111100000000111111111111111100000000000000001111111111111111111111110000000000000000000000001111111111111111111111111111111100000000000000000000000000000000101100111000111100001111100000111111000000111111100000001111111100000000");

    println!("a_biguint = {}", a_biguint.to_string_with_radix(36).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(36).unwrap(), "4993ID4SD9M4DT2QO9EF7VZKGDLH3SY0SOW4CHRKE5CQA40MPS");

    println!("a_biguint = {}", a_biguint.to_string_with_radix(62).unwrap());
    assert_eq!(a_biguint.to_string_with_radix(62).unwrap(), "eV7xhnHDmgsyLnqm5P9ZaJfdOP07xlqS2DaBiV2F7dg");
  
    match a_biguint.to_string_with_radix(1)
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }

    match a_biguint.to_string_with_radix(63)
    {
        Ok(txt) => { println!("a_biguint = {}", txt); },
        Err(e) => { assert_eq!(e, NumberErr::OutOfValidRadixRange) },
    }
    println!("---------------------------");
}


fn biguint_flag_manipulation_main()
{
    biguint_set_overflow();
    biguint_reset_overflow();
    biguint_is_overflow();
    biguint_set_underflow();
    biguint_reset_underflow();
    biguint_is_underflow();
    biguint_set_infinity();
    biguint_reset_infinity();
    biguint_is_infinity();
    biguint_set_divided_by_zero();
    biguint_reset_divided_by_zero();
    biguint_is_divided_by_zero();
    biguint_set_undefined();
    biguint_reset_undefined();
    biguint_is_undefined();
    biguint_set_left_carry();
    biguint_reset_left_carry();
    biguint_is_left_carry();
    biguint_set_right_carry();
    biguint_reset_right_carry();
    biguint_is_right_carry();
}

fn biguint_set_overflow()
{
    println!("biguint_set_overflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_overflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_overflow()
{
    println!("biguint_reset_overflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_overflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_overflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_overflow()
{
    println!("biguint_is_overflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_overflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_underflow()
{
    println!("biguint_set_underflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_underflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_underflow()
{
    println!("biguint_reset_underflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_underflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_underflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_underflow()
{
    println!("biguint_is_underflow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_underflow();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_infinity()
{
    println!("biguint_set_infinity()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_infinity();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_infinity()
{
    println!("biguint_reset_infinity()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_infinity();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_infinity();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_infinity()
{
    println!("biguint_is_infinity()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_infinity();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_divided_by_zero()
{
    println!("biguint_set_divided_by_zero()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_divided_by_zero();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_divided_by_zero()
{
    println!("biguint_reset_divided_by_zero()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_divided_by_zero();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_divided_by_zero();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_divided_by_zero()
{
    println!("biguint_is_divided_by_zero()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_divided_by_zero();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_undefined()
{
    println!("biguint_set_undefined()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_undefined();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_undefined()
{
    println!("biguint_reset_undefined()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);

    a_biguint.set_undefined();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_undefined();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_undefined()
{
    println!("biguint_is_undefined()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_undefined();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_left_carry()
{
    println!("biguint_set_left_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_left_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_reset_left_carry()
{
    println!("biguint_reset_left_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_left_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.reset_left_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_left_carry()
{
    println!("biguint_is_left_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_left_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), true);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_set_right_carry()
{
    println!("biguint_set_right_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_right_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);
    println!("---------------------------");
}

fn biguint_reset_right_carry()
{
    println!("biguint_reset_right_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_right_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);

    a_biguint.reset_right_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_right_carry()
{
    println!("biguint_is_right_carry()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.set_right_carry();
    println!("a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), true);
    println!("---------------------------");
}
