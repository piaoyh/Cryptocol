// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]



pub fn main()
{
    random_specific_quick_start();
    random_constructors();
}

fn random_specific_quick_start()
{
    random_random_big_keccak_1024_quick_start();
    random_random_sha3_512_quick_start();
    random_random_sha2_512_quick_start();
    random_any_shake_256_quick_start();
    random_any_shake_128_quick_start();
    random_any_sha3_512_quick_start();
    random_any_sha3_256_quick_start();
    random_any_sha2_512_quick_start();
    random_any_sha2_256_quick_start();
    random_slapdash_sha1_quick_start();
    random_slapdash_sha0_quick_start();
    random_slapdash_md5_quick_start();
    random_slapdash_md4_quick_start();
    random_random_riindael_quick_start();
    random_any_riindael_quick_start();
    random_slapdash_des_quick_start();
    random_slapdash_num_c_quick_start();
}

fn random_constructors()
{
    random_new();
    random_new_with_seeds();
    random_new_with_seed_arrarys();
    random_new_with_seed_collector();
    random_new_with_seed_collector_seeds();
    random_new_with_seed_collector_seed_arrays();
}

fn random_random_big_keccak_1024_quick_start()
{
    println!("random_random_big_keccak_1024_quick_start");
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create();
    println!("Random_PRNG_Creator number = {}", rand.random_u128());
    println!("Random_PRNG_Creator number = {}", rand.random_u64());
    println!("Random_PRNG_Creator number = {}", rand.random_u32());
    println!("Random_PRNG_Creator number = {}", rand.random_u16());
    println!("Random_PRNG_Creator number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random_PRNG_Creator number u16 = {}", num); }

    println!("Random_PRNG_Creator odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random_PRNG_Creator odd number u16 = {}", num); }

    println!("Random_PRNG_Creator 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random_PRNG_Creator 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random_PRNG_Creator prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random_PRNG_Creator usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_random_sha3_512_quick_start()
{
    println!("random_random_sha3_512_quick_start");
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random_PRNG_Creator_SHA3_512::create();
    println!("Random_PRNG_Creator number = {}", rand.random_u128());
    println!("Random_PRNG_Creator number = {}", rand.random_u64());
    println!("Random_PRNG_Creator number = {}", rand.random_u32());
    println!("Random_PRNG_Creator number = {}", rand.random_u16());
    println!("Random_PRNG_Creator number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random_PRNG_Creator number u16 = {}", num); }

    println!("Random_PRNG_Creator odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random_PRNG_Creator odd number u16 = {}", num); }

    println!("Random_PRNG_Creator 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random_PRNG_Creator 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random_PRNG_Creator prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random_PRNG_Creator usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_random_sha2_512_quick_start()
{
    println!("random_random_sha2_512_quick_start");
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random_PRNG_Creator_SHA2_512::create();
    println!("Random_PRNG_Creator number = {}", rand.random_u128());
    println!("Random_PRNG_Creator number = {}", rand.random_u64());
    println!("Random_PRNG_Creator number = {}", rand.random_u32());
    println!("Random_PRNG_Creator number = {}", rand.random_u16());
    println!("Random_PRNG_Creator number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random_PRNG_Creator number u16 = {}", num); }

    println!("Random_PRNG_Creator odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random_PRNG_Creator odd number u16 = {}", num); }

    println!("Random_PRNG_Creator 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random_PRNG_Creator 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random_PRNG_Creator prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random_PRNG_Creator usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_any_shake_256_quick_start()
{
    println!("random_any_shake_256_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHAKE_256::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_any_shake_128_quick_start()
{
    println!("random_any_shake_128_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHAKE_128::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_any_sha3_512_quick_start()
{
    println!("random_any_sha3_512_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHA3_512::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_any_sha3_256_quick_start()
{
    println!("random_any_sha3_256_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHA3_256::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_any_sha2_512_quick_start()
{
    println!("random_any_sha2_512_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHA2_512::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_any_sha2_256_quick_start()
{
    println!("random_any_sha2_256_quick_start");
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_SHA2_256::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_slapdash_sha1_quick_start()
{
    println!("random_slapdash_sha1_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Random_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_slapdash_sha0_quick_start()
{
    println!("random_slapdash_sha0_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Slapdash_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}



fn random_slapdash_md5_quick_start()
{
    println!("random_slapdash_md5_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_MD5::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Slapdash_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_slapdash_md4_quick_start()
{
    println!("random_slapdash_md4_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_MD4::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Slapdash_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_random_riindael_quick_start()
{
    println!("random_random_riindael_quick_start");
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random_PRNG_Creator_AES_128::create();
    println!("Random_PRNG_Creator number = {}", rand.random_u128());
    println!("Random_PRNG_Creator number = {}", rand.random_u64());
    println!("Random_PRNG_Creator number = {}", rand.random_u32());
    println!("Random_PRNG_Creator number = {}", rand.random_u16());
    println!("Random_PRNG_Creator number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random_PRNG_Creator number u16 = {}", num); }

    println!("Random_PRNG_Creator odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random_PRNG_Creator odd number u16 = {}", num); }

    println!("Random_PRNG_Creator 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random_PRNG_Creator 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random_PRNG_Creator prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random_PRNG_Creator usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_any_riindael_quick_start()
{
    println!("random_any_riindael_quick_start");
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut any = Any_PRNG_Creator_AES_128::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());
    println!("Any_PRNG_Creator number = {}", any.random_u64());
    println!("Any_PRNG_Creator number = {}", any.random_u32());
    println!("Any_PRNG_Creator number = {}", any.random_u16());
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any_PRNG_Creator number u16 = {}", num); }

    println!("Any_PRNG_Creator odd number usize = {}", any.random_odd_uint::<usize>());
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any_PRNG_Creator odd number u16 = {}", num); }

    println!("Any_PRNG_Creator 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any_PRNG_Creator 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any_PRNG_Creator prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any_PRNG_Creator usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = any.random_array();
    for i in 0..20
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    any.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Any_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = any.random_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = any.random_with_msb_set_biguint();
    println!("Any_PRNG_Creator Number: {}", biguint);

    biguint = any.random_odd_with_msb_set_biguint();
    println!("512-bit Any_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Any_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_slapdash_des_quick_start()
{
    println!("random_slapdash_des_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_DES::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Slapdash_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_slapdash_num_c_quick_start()
{
    println!("random_slapdash_num_c_quick_start");
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash_PRNG_Creator number u64 = {}", num); }

    if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
        { println!("Slapdash_PRNG_Creator number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator odd number usize = {}", slapdash.random_odd_uint::<usize>());
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash_PRNG_Creator odd number u16 = {}", num); }

    println!("Slapdash_PRNG_Creator 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash_PRNG_Creator 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash_PRNG_Creator prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash_PRNG_Creator usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = slapdash.random_array();
    for i in 0..20
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    slapdash.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Slapdash_PRNG_Creator number {} => {}", i, num[i]); }

    let mut biguint: U512 = slapdash.random_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash_PRNG_Creator Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash_PRNG_Creator odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = slapdash.random_with_msb_set_biguint();
    println!("Slapdash_PRNG_Creator Number: {}", biguint);

    biguint = slapdash.random_odd_with_msb_set_biguint();
    println!("512-bit Slapdash_PRNG_Creator Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
    println!("Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Slapdash_PRNG_Creator Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_new()
{
    println!("random_new");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let mut rand = Random_PRNG_Creator::create();
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let mut any = Any_PRNG_Creator::create();
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create();
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let mut rand = Random_PRNG_Creator_SHA3_512::create();
    let num: U768 = rand.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let mut rand = Random_PRNG_Creator_SHA2_512::create();
    let num: U512 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let mut any = Any_PRNG_Creator_SHAKE_256::create();
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let mut any = Any_PRNG_Creator_SHAKE_128::create();
    println!("Any_PRNG_Creator number = {}", any.random_u128());

    // Example for Any_PRNG_Creator_SHA3_512
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    let mut any = Any_PRNG_Creator_SHA3_512::create();
    println!("Any_PRNG_Creator number = {}", any.random_u64());

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let mut any = Any_PRNG_Creator_SHA3_256::create();
    println!("Any_PRNG_Creator number = {}", any.random_u32());

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let mut any = Any_PRNG_Creator_SHA2_512::create();
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let mut any = Any_PRNG_Creator_SHA2_256::create();
    println!("Any_PRNG_Creator number = {}", any.random_u8());

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_usize());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u16());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let mut rand = Random_PRNG_Creator_AES_128::create();
    let num: U512 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let mut any = Any_PRNG_Creator_AES_128::create();
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let mut slapdash = Slapdash_PRNG_Creator_DES::create();
    let num: U256 = slapdash.random_odd_biguint();
    println!("Slapdash_PRNG_Creator number = {}", num);

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_usize());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let mut slapdash = Slapdash_PRNG_Creator::create();
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_seeds()
{
    println!("random_new_with_seeds");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let mut rand = Random_PRNG_Creator::create_with_seeds(10500872879054459758_u64, 15887751380961987625_u64);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let mut any = Any_PRNG_Creator::create_with_seeds(100, 25);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seeds(0, 0);
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seeds(u64::MAX, u64::MAX);
    let num: U768 = rand.random_odd_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seeds(15698731215687456325, 10684237915728469725);
    let num: U256 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let mut any = Any_PRNG_Creator_SHAKE_256::create_with_seeds(123456789, 987654321);
    let num: U512 = any.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let mut any = Any_PRNG_Creator_SHAKE_128::create_with_seeds(u32::MAX as u64, u32::MAX as u64);
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_512
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    let mut any = Any_PRNG_Creator_SHA3_512::create_with_seeds(u64::MAX, u64::MAX);
    let num: U768 = any.random_odd_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let mut any = Any_PRNG_Creator_SHA3_256::create_with_seeds(u64::MAX, u64::MAX);
    let num: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let mut any = Any_PRNG_Creator_SHA2_512::create_with_seeds(2879054410500759758, 15887876257513809619);
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let mut any = Any_PRNG_Creator_SHA2_256::create_with_seeds(610458805, 215793685);
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create_with_seeds(18782, 50558);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_uint::<u8>());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create_with_seeds(0, 125);
    println!("Slapdash_PRNG_Creator prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create_with_seeds(58, 161);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create_with_seeds(106842379157284697, 18446744073709551615);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let mut rand = Random_PRNG_Creator_AES_128::create_with_seeds(112233445566778899, 998877665544332211);
    println!("Random_PRNG_Creator number = {}", rand.random_u32());

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let mut any = Any_PRNG_Creator_AES_128::create_with_seeds(u16::MAX as u64, u16::MAX as u64);
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let mut slapdash = Slapdash_PRNG_Creator_DES::create_with_seeds(u8::MAX as u64, u8::MAX as u64);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create_with_seeds(458861005, 793621585);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let mut slapdash = Slapdash_PRNG_Creator::create_with_seeds(50558, 18782);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("-------------------------------");
}

fn random_new_with_seed_arrarys()
{
    println!("random_new_with_seed_arrarys");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator::create_with_seed_arrays(seed, aux);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let seed = [12_u64, 123456789_u64, 10500872879054459758_u64, 987654321_u64, 777777777777_u64, 852648791354687_u64, 555555555555_u64, 741258963_u64];
    let aux = [789456123_u64, 15887751380961987625_u64, 5_u64, 9632587414_u64, 789654123_u64, 369258147_u64, 58976541235_u64, 9513574682_u64];
    let mut any = Any_PRNG_Creator::create_with_seed_arrays(seed, aux);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
    let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_arrays(seed, aux);
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let seed = [123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 12_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_arrays(seed, aux);
    let num: U768 = rand.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let seed = [10500872879054459758_u64, 777777777777_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789654123_u64, 5_u64, 789456123_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA3_256::create_with_seed_arrays(seed, aux);
    let num: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 555555555555_u64, 852648791354687_u64, 777777777777_u64, 741258963_u64];
    let aux = [1789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 5887751380961987625_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHAKE_256::create_with_seed_arrays(seed, aux);
    let num: U512 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHAKE_128::create_with_seed_arrays(seed, aux);
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_arrays(seed, aux);
    let num: U256 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA2_512::create_with_seed_arrays(seed, aux);
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA2_256::create_with_seed_arrays(seed, aux);
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_uint::<u8>());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_AES_128::create_with_seed_arrays(seed, aux);
    println!("Any_PRNG_Creator number = {}", rand.random_u32());

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_AES_128::create_with_seed_arrays(seed, aux);
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_DES::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator::create_with_seed_arrays(seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("-------------------------------");
}

fn random_new_with_seed_collector()
{
    println!("random_new_with_seed_collector");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    fn seed_collector() -> [u64; 8]
    {
        use std::time::{ SystemTime, UNIX_EPOCH };
        use cryptocol::number::LongerUnion;

        let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
        let mut seed_buffer = [ptr; 8];
        for i in 0..8
            { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }

        if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
        {
            let common = LongerUnion::new_with(nanos.as_nanos());
            for i in 0..4
            {
                let j = i << 1;
                seed_buffer[j] = common.get_ulong_(0);
                seed_buffer[j + 1] = common.get_ulong_(1);
            }
        }
        seed_buffer
    }

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let mut rand = Random_PRNG_Creator::create_with_seed_collector(seed_collector);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let mut any = Any_PRNG_Creator::create_with_seed_collector(seed_collector);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector(seed_collector);
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector(seed_collector);
    let num: U768 = rand.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_512
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector(seed_collector);
    let num: U512 = any.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let mut any = Any_PRNG_Creator_SHA3_256::create_with_seed_collector(seed_collector);
    let num: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let mut any = Any_PRNG_Creator_SHAKE_256::create_with_seed_collector(seed_collector);
    let num: U512 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let mut any = Any_PRNG_Creator_SHAKE_128::create_with_seed_collector(seed_collector);
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector(seed_collector);
    let num: U256 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let mut any = Any_PRNG_Creator_SHA2_512::create_with_seed_collector(seed_collector);
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let mut any = Any_PRNG_Creator_SHA2_256::create_with_seed_collector(seed_collector);
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_uint::<u8>());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let mut rand = Random_PRNG_Creator_AES_128::create_with_seed_collector(seed_collector);
    println!("Any_PRNG_Creator number = {}", rand.random_u32());

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let mut any = Any_PRNG_Creator_AES_128::create_with_seed_collector(seed_collector);
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let mut slapdash = Slapdash_PRNG_Creator_DES::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let mut slapdash = Slapdash_PRNG_Creator::create_with_seed_collector(seed_collector);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("-------------------------------");
}

fn random_new_with_seed_collector_seeds()
{
    println!("random_new_with_seed_collector_seeds");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    fn seed_collector() -> [u64; 8]
    {
        use std::time::{ SystemTime, UNIX_EPOCH };
        use cryptocol::number::LongerUnion;

        let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
        let mut seed_buffer = [ptr; 8];
        for i in 0..8
            { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }

        if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
        {
            let common = LongerUnion::new_with(nanos.as_nanos());
            for i in 0..4
            {
                let j = i << 1;
                seed_buffer[j] = common.get_ulong_(0);
                seed_buffer[j + 1] = common.get_ulong_(1);
            }
        }
        seed_buffer
    }

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let mut rand = Random_PRNG_Creator::create_with_seed_collector_seeds(seed_collector, 10500872879054459758_u64, 15887751380961987625_u64);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let mut any = Any_PRNG_Creator::create_with_seed_collector_seeds(seed_collector, 100, 25);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seeds(seed_collector, 0, 0);
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector_seeds(seed_collector, u64::MAX, u64::MAX);
    let num: U768 = rand.random_odd_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector_seeds(seed_collector, 15698731215687456325, 10684237915728469725);
    let num: U256 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let mut any = Any_PRNG_Creator_SHAKE_256::create_with_seed_collector_seeds(seed_collector, 123456789, 987654321);
    let num: U512 = any.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let mut any = Any_PRNG_Creator_SHAKE_128::create_with_seed_collector_seeds(seed_collector, u32::MAX as u64, u32::MAX as u64);
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_512
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector_seeds(seed_collector, u64::MAX, u64::MAX);
    let num: U768 = any.random_odd_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let mut any = Any_PRNG_Creator_SHA3_256::create_with_seed_collector_seeds(seed_collector, u64::MAX, u64::MAX);
    let num: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let mut any = Any_PRNG_Creator_SHA2_512::create_with_seed_collector_seeds(seed_collector, 2879054410500759758, 15887876257513809619);
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let mut any = Any_PRNG_Creator_SHA2_256::create_with_seed_collector_seeds(seed_collector, 610458805, 215793685);
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create_with_seed_collector_seeds(seed_collector, 18782, 50558);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_uint::<u8>());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create_with_seed_collector_seeds(seed_collector, 0, 125);
    println!("Slapdash_PRNG_Creator prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create_with_seed_collector_seeds(seed_collector, 58, 161);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create_with_seed_collector_seeds(seed_collector, 106842379157284697, 18446744073709551615);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let mut rand = Random_PRNG_Creator_AES_128::create_with_seed_collector_seeds(seed_collector, 112233445566778899, 998877665544332211);
    println!("Random_PRNG_Creator number = {}", rand.random_u32());

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let mut any = Any_PRNG_Creator_AES_128::create_with_seed_collector_seeds(seed_collector, u16::MAX as u64, u16::MAX as u64);
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let mut slapdash = Slapdash_PRNG_Creator_DES::create_with_seed_collector_seeds(seed_collector, u8::MAX as u64, u8::MAX as u64);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create_with_seed_collector_seeds(seed_collector, 458861005, 793621585);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let mut slapdash = Slapdash_PRNG_Creator::create_with_seed_collector_seeds(seed_collector, 50558, 18782);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("-------------------------------");
}

fn random_new_with_seed_collector_seed_arrays()
{
    println!("random_new_with_seed_collector_seed_arrays");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    fn seed_collector() -> [u64; 8]
    {
        use std::time::{ SystemTime, UNIX_EPOCH };
        use cryptocol::number::LongerUnion;

        let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
        let mut seed_buffer = [ptr; 8];
        for i in 0..8
            { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }

        if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
        {
            let common = LongerUnion::new_with(nanos.as_nanos());
            for i in 0..4
            {
                let j = i << 1;
                seed_buffer[j] = common.get_ulong_(0);
                seed_buffer[j + 1] = common.get_ulong_(1);
            }
        }
        seed_buffer
    }

    // Example for Random_PRNG_Creator
    use cryptocol::random::Random_PRNG_Creator;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator
    use cryptocol::random::Any_PRNG_Creator;
    let seed = [12_u64, 123456789_u64, 10500872879054459758_u64, 987654321_u64, 777777777777_u64, 852648791354687_u64, 555555555555_u64, 741258963_u64];
    let aux = [789456123_u64, 15887751380961987625_u64, 5_u64, 9632587414_u64, 789654123_u64, 369258147_u64, 58976541235_u64, 9513574682_u64];
    let mut any = Any_PRNG_Creator::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_BIG_KECCAK_1024
    use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
    let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA3_512
    use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    let seed = [123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 12_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U768 = rand.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_512
    use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    let seed = [12_u64, 123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [9513574682_u64, 9632587414_u64, 15887751380961987625_u64, 789456123_u64, 58976541235_u64, 369258147_u64, 789654123_u64, 5_u64];
    let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U512 = any.random_odd_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA3_256
    use cryptocol::random::Any_PRNG_Creator_SHA3_256;
    let seed = [10500872879054459758_u64, 777777777777_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789654123_u64, 5_u64, 789456123_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA3_256::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_256
    use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 555555555555_u64, 852648791354687_u64, 777777777777_u64, 741258963_u64];
    let aux = [1789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 5887751380961987625_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHAKE_256::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U512 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHAKE_128
    use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHAKE_128::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U384 = any.random_biguint();
    println!("Any_PRNG_Creator number = {}", num);

    // Example for Random_PRNG_Creator_SHA2_512
    use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    let num: U256 = rand.random_biguint();
    println!("Random_PRNG_Creator number = {}", num);

    // Example for Any_PRNG_Creator_SHA2_512
    use cryptocol::random::Any_PRNG_Creator_SHA2_512;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA2_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Any_PRNG_Creator_SHA2_256
    use cryptocol::random::Any_PRNG_Creator_SHA2_256;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_SHA2_256::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any_PRNG_Creator number = {}", num); }

    // Example for Slapdash_PRNG_Creator_SHA1
    use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_SHA1::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_uint::<u8>());

    // Example for Slapdash_PRNG_Creator_SHA0
    use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_SHA0::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));

    // Example for Slapdash_PRNG_Creator_MD5
    use cryptocol::random::Slapdash_PRNG_Creator_MD5;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_MD5::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u128());

    // Example for Slapdash_PRNG_Creator_MD4
    use cryptocol::random::Slapdash_PRNG_Creator_MD4;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_MD4::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Random_PRNG_Creator_AES_128
    use cryptocol::random::Random_PRNG_Creator_AES_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut rand = Random_PRNG_Creator_AES_128::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Any_PRNG_Creator number = {}", rand.random_u32());

    // Example for Any_PRNG_Creator_AES_128
    use cryptocol::random::Any_PRNG_Creator_AES_128;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut any = Any_PRNG_Creator_AES_128::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Any_PRNG_Creator number = {}", any.random_u16());

    // Example for Slapdash_PRNG_Creator_DES
    use cryptocol::random::Slapdash_PRNG_Creator_DES;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_DES::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u8());

    // Example for Slapdash_PRNG_Creator_CPRNG_Engine
    use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u64());

    // Example for Slapdash_PRNG_Creator
    use cryptocol::random::Slapdash_PRNG_Creator;
    let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    let mut slapdash = Slapdash_PRNG_Creator::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    println!("Slapdash_PRNG_Creator number = {}", slapdash.random_u32());
    println!("-------------------------------");
}
