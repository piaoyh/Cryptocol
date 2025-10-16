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
    random_quick_start();
    random_new();
    random_new_with();
    random_new_with_generators_seeds();
    random_new_with_generators_seed_array();
    random_new_with_generators_seed_collector();
    random_new_with_generators_seed_collector_seeds();
    random_new_with_generators_seed_collector_seed_arrays();
    random_random_u8();
    random_random_u16();
    random_random_u32();
    random_random_u64();
    random_random_u128();
    random_random_usize();
    random_random_uint();
    random_random_under_uint();
    random_random_under_uint_();
    random_random_minmax_uint();
    random_random_minmax_uint_();
    random_random_odd_uint();
    random_random_odd_under_uint();
    random_random_odd_under_uint_();
    random_random_with_msb_set_uint();
    random_random_odd_with_msb_set_uint();
    random_random_prime_using_miller_rabin_uint();
    random_random_prime_with_msb_set_using_miller_rabin_uint();
    random_random_array();
    random_put_random_in_array();
    random_random_biguint();
    random_random_under_biguint();
    random_random_under_biguint_();
    random_random_odd_biguint();
    random_random_odd_under_biguint();
    random_random_odd_under_biguint_();
    random_random_with_msb_set_biguint();
    random_random_odd_with_msb_set_biguint();
    random_random_prime_using_miller_rabin_biguint();
    random_random_prime_with_msb_set_using_miller_rabin_biguint();
}

fn random_quick_start()
{
    println!("random_quick_start");
    use cryptocol::random::Random;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut rand = Random::new();
    println!("Random number = {}", rand.random_u128());
    println!("Random number = {}", rand.random_u64());
    println!("Random number = {}", rand.random_u32());
    println!("Random number = {}", rand.random_u16());
    println!("Random number = {}", rand.random_u8());

    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random number u64 = {}", num); }

    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random number u16 = {}", num); }

    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }

    println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    let num: [u128; 20] = rand.random_array();
    for i in 0..20
        { println!("Random number {} => {}", i, num[i]); }

    let mut num = [0_u64; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random number {} => {}", i, num[i]); }

    let mut biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);

    let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    ceiling = U1024::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    biguint = rand.random_with_msb_set_biguint();
    println!("Random Number: {}", biguint);

    biguint = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random Odd Number = {}", biguint);
    assert!(biguint > U512::halfmax());
    assert!(biguint.is_odd());

    biguint = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random Prime Number = {}", biguint);
    assert!(biguint.is_odd());

    biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random Prime Number = {}", biguint);
    assert!(biguint.is_odd());
    println!("-------------------------------");
}

fn random_new()
{
    println!("random_new");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);
    
    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let num: U1024 = rand.random_with_msb_set_biguint();
    println!("Random number = {}", num);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let num: U512 = rand.random_biguint();
    println!("Random number = {}", num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);
    
    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any number = {}", any.random_u128());
    
    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any number = {}", any.random_u64());
    
    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any number = {}", any.random_u32());
    
    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any number = {}", any.random_u16());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any number = {}", any.random_u8());
    
    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash number = {}", slapdash.random_usize());
    
    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash number = {}", slapdash.random_u64());
    
    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash number = {}", slapdash.random_u32());
    
    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash number = {}", slapdash.random_u16());
    
    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num: U512 = rand.random_with_msb_set_biguint();
    println!("Random number = {}", num);
    
    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);
    
    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash number = {}", slapdash.random_odd_biguint());
}

fn random_new_with()
{
    println!("random_new_with");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let mut rand = RandGen::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new());
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut any = AnyGen::new_with(SHA3_512::new(), SHA3_512::new());
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut any = AnyGen::new_with(SHA2_512::new(), SHA2_512::new());
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut rand = RandGen::new_with(SHAKE_256::new(), SHAKE_256::new());
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut any = AnyGen::new_with(SHAKE_128::new(), SHAKE_128::new());
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut any = AnyGen::new_with(SHA3_256::new(), SHA3_256::new());
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut any = AnyGen::new_with(SHA2_256::new(), SHA2_256::new());
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let mut any = SlapdashGen::new_with(SHA1::new(), SHA0::new());
    println!("Any number = {}", any.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let mut any = SlapdashGen::new_with(MD5::new(), MD4::new());
    println!("Any number = {}", any.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let mut rand = RandGen::new_with(AES_128::new(), AES_128::new());
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let mut slapdash = SlapdashGen::new_with(DES::new(), DES::new());
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_generators_seeds()
{
    println!("random_new_with_generators_seeds");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let mut rand = RandGen::new_with_generators_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), 10500872879054459758_u64, 15887751380961987625_u64);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut any = AnyGen::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), 100, 25);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut any = AnyGen::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), 0, 0);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut rand = RandGen::new_with_generators_seeds(SHAKE_256::new(), SHAKE_256::new(), u64::MAX, u64::MAX);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut any = AnyGen::new_with_generators_seeds(SHAKE_128::new(), SHAKE_128::new(), 123456789, 987654321);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut any = AnyGen::new_with_generators_seeds(SHA3_256::new(), SHA3_256::new(), u32::MAX as u64, u32::MAX as u64);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut any = AnyGen::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), 15698731215687456325, 10684237915728469725);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let mut slapdash = SlapdashGen::new_with_generators_seeds(SHA1::new(), SHA0::new(), 2879054410500759758, 15887876257513809619);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let mut slapdash = SlapdashGen::new_with_generators_seeds(MD5::new(), MD4::new(), 610458805, 215793685);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let mut rand = RandGen::new_with_generators_seeds(AES_128::new(), AES_128::new(), 18782, 50558);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let mut slapdash = SlapdashGen::new_with_generators_seeds(DES::new(), DES::new(), 0, 125);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}
fn random_new_with_generators_seed_array()
{
    println!("random_new_with_generators_seed_array");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed, aux);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed, aux);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed, aux);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed, aux);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed, aux);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed, aux);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(SHA1::new(), SHA0::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(MD5::new(), MD4::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(DES::new(), DES::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector()
{
    println!("random_new_with_generators_seed_collector");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
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

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let mut rand = RandGen::new_with_generators_seed_collector(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut any = AnyGen::new_with_generators_seed_collector(SHA3_512::new(), SHA3_512::new(), seed_collector);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut any = AnyGen::new_with_generators_seed_collector(SHA2_512::new(), SHA2_512::new(),seed_collector);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut rand = RandGen::new_with_generators_seed_collector(SHAKE_256::new(), SHAKE_256::new(), seed_collector);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut any = AnyGen::new_with_generators_seed_collector(SHAKE_128::new(), SHAKE_128::new(), seed_collector);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut any = AnyGen::new_with_generators_seed_collector(SHA3_256::new(), SHA3_256::new(), seed_collector);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut any = AnyGen::new_with_generators_seed_collector(SHA2_256::new(), SHA2_256::new(), seed_collector);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector(SHA1::new(), SHA0::new(), seed_collector);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector(MD5::new(), MD4::new(), seed_collector);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let mut rand = RandGen::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector(DES::new(), DES::new(), seed_collector);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector_seeds()
{
    println!("random_new_with_generators_seed_collector_seeds");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let mut rand = RandGen::new_with_generators_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), 10500872879054459758_u64, 15887751380961987625_u64);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut any = AnyGen::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), 100, 25);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut any = AnyGen::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), 0, 0);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut rand = RandGen::new_with_generators_seeds(SHAKE_256::new(), SHAKE_256::new(), u64::MAX, u64::MAX);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut any = AnyGen::new_with_generators_seeds(SHAKE_128::new(), SHAKE_128::new(), 123456789, 987654321);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut any = AnyGen::new_with_generators_seeds(SHA3_256::new(), SHA3_256::new(), u32::MAX as u64, u32::MAX as u64);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut any = AnyGen::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), 15698731215687456325, 10684237915728469725);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let mut slapdash = SlapdashGen::new_with_generators_seeds(SHA1::new(), SHA0::new(), 2879054410500759758, 15887876257513809619);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let mut slapdash = SlapdashGen::new_with_generators_seeds(MD5::new(), MD4::new(), 610458805, 215793685);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let mut rand = RandGen::new_with_generators_seeds(AES_128::new(), AES_128::new(), 18782, 50558);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let mut slapdash = SlapdashGen::new_with_generators_seeds(DES::new(), DES::new(), 0, 125);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector_seed_arrays()
{
    println!("random_new_with_generators_seed_collector_seed_arrays");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for BIG_KECCAK_1024
    use cryptocol::hash::BIG_KECCAK_1024;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed, aux);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed, aux);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed, aux);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed, aux);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed, aux);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed, aux);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(SHA1::new(), SHA0::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(MD5::new(), MD4::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let seed = [10500872879054459758_u64, 54459758105008728790, 28790544591050087758, 87281050044597905758, 45900810579072854758, 10572800879059744558, 59758728710500905448, 79054105075808728459];
    let aux = [10500054459758872879_u64, 75810500854459728790, 28790877585445910500, 50044597872810905758, 40579072855900814758, 74410572800879059558, 87105448597050095872, 58087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(DES::new(), DES::new(), seed, aux);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_random_u8()
{
    println!("random_random_u8");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u8()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u8()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u8()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u8()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u8()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u8()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u8()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u8()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u8()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u8()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u8()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u8()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u8()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u8()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u8()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u8()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u8()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u8()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u8()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u8()); }
    println!("-------------------------------");
}

fn random_random_u16()
{
    println!("random_random_u16");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u16()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u16()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u16()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u16()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u16()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u16()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u16()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u16()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u16()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u16()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_u16()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u16()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u16()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u16()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u16()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u16()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u16()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u16()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u16()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u16()); }
    println!("-------------------------------");
}

fn random_random_u32()
{
    println!("random_random_u32");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u32()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u32()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u32()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u32()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u32()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u32()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u32()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u32()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u32()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u32()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u32()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u32()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u32()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u32()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u32()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u32()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u32()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u32()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u32()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u32()); }
    println!("-------------------------------");
}

fn random_random_u64()
{
    println!("random_random_u64");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u64()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u64()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u64()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u64()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u64()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u64()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u64()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u64()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u64()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u64()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_u64()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u64()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u64()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u64()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u64()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u64()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u64()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u64()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u64()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u64()); }
    println!("-------------------------------");
}

fn random_random_u128()
{
    println!("random_random_u128");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_u128()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_u128()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u128()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u128()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u128()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u128()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u128()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u128()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u128()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u128()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_u128()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u128()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u128()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u128()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u128()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u128()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u128()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u128()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u128()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u128()); }
    println!("-------------------------------");
}

fn random_random_usize()
{
    println!("random_random_usize");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_usize()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_usize()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_usize()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_usize()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_usize()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_usize()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_usize()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_usize()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_usize()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_usize()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_usize()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_usize()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_usize()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_usize()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_usize()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_usize()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_usize()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_usize()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_usize()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_usize()); }
    println!("-------------------------------");
}

fn random_random_uint()
{
    println!("random_random_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    for i in 0..10
        { println!("{} Random number (Random) = {}", i, rand.random_uint::<u8>()); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    for i in 0..10
        { println!("{} Any number (Any) = {}", i, any.random_uint::<u16>()); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    for i in 0..10
        { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_uint::<u32>()); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_uint::<u64>()); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    for i in 0..10
        { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_uint::<u128>()); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_uint::<usize>()); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    for i in 0..10
        { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_uint::<u16>()); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_uint::<u32>()); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    for i in 0..10
        { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_uint::<u64>()); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_uint::<u128>()); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    for i in 0..10
        { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_uint::<u8>()); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_uint::<usize>()); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_uint::<u32>()); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_uint::<u64>()); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_uint::<u128>()); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    for i in 0..10
        { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_uint::<u8>()); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    for i in 0..10
        { println!("{} Any number (Any_Rijndael) = {}", i, any.random_uint::<u16>()); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_uint::<usize>()); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_uint::<u64>()); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    for i in 0..10
        { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_uint::<u128>()); }
    println!("-------------------------------");
}

fn random_random_under_uint()
{
    println!("random_random_under_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    if let Some(num) = rand.random_under_uint(12_u8)
        { println!("Random number u8 = {}", num); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any number u16 = {}", num); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    if let Some(num) = rand.random_under_uint(12345678_u32)
        { println!("Random number u32 = {}", num); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    if let Some(num) = rand.random_under_uint(1234567890123456_u64)
        { println!("Random number u64 = {}", num); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    if let Some(num) = rand.random_under_uint(12345678901234567890_u128)
        { println!("Random number u128 = {}", num); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    if let Some(num) = any.random_under_uint(1234_usize)
        { println!("Any number usize = {}", num); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    if let Some(num) = any.random_under_uint(0_usize)
        { println!("Any number usize = {}", num); }
    else
        { println!("No any unsigned number under 0!"); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    if let Some(num) = any.random_under_uint(12_u8)
        { println!("Any number u8 = {}", num); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    if let Some(num) = any.random_under_uint(1234_u16)
        { println!("Any number u16 = {}", num); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    if let Some(num) = any.random_under_uint(12345678_u32)
        { println!("Any number u32 = {}", num); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    if let Some(num) = any.random_under_uint(1234567890123456_u64)
        { println!("Any number u64 = {}", num); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    if let Some(num) = slapdash.random_under_uint(12345678901234567890_u128)
        { println!("Slapdash number u128 = {}", num); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    if let Some(num) = slapdash.random_under_uint(1234_usize)
        { println!("Slapdash number usize = {}", num); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    if let Some(num) = slapdash.random_under_uint(0_u64)
        { println!("Slapdash number usize = {}", num); }
    else
        { println!("No slapdash unsigned number under 0!"); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    if let Some(num) = slapdash.random_under_uint(12_u8)
        { println!("Slapdash number u8 = {}", num); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    if let Some(num) = rand.random_under_uint(1234_u16)
        { println!("Random number u16 = {}", num); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    if let Some(num) = any.random_under_uint(12345678_u32)
        { println!("Any number u32 = {}", num); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
        { println!("Slapdash number u64 = {}", num); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    if let Some(num) = slapdash.random_under_uint(12345678901234567890_u128)
        { println!("Slapdash number u128 = {}", num); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    if let Some(num) = slapdash.random_under_uint(0_u32)
        { println!("Slapdash number usize = {}", num); }
    else
        { println!("No slapdash unsigned number under 0!"); }
    println!("-------------------------------");
}

fn random_random_under_uint_()
{
    println!("random_random_under_uint_");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let num = rand.random_under_uint_(12_u8);
    println!("Random number u8 = {}", num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let num = any.random_under_uint_(1234_u16);
    println!("Any number u16 = {}", num);

    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let num = rand.random_under_uint_(12345678_u32);
    println!("Random number u32 = {}", num);

    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let num = rand.random_under_uint_(1234567890123456_u64);
    println!("Random number u64 = {}", num);

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let num = rand.random_under_uint_(12345678901234567890_u128);
    println!("Random number u128 = {}", num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num = any.random_under_uint_(1234_usize);
    println!("Any number usize = {}", num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let num = any.random_under_uint_(12_u8);
    println!("Any number u8 = {}", num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let num = any.random_under_uint_(1234_u16);
    println!("Any number u16 = {}", num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let num = any.random_under_uint_(12345678_u32);
    println!("Any number u32 = {}", num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let num = any.random_under_uint_(1234567890123456_u64);
    println!("Any number u64 = {}", num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let num = any.random_under_uint_(12345678901234567890_u128);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let num = slapdash.random_under_uint_(1234_usize);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let num = slapdash.random_under_uint_(12_u8);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let num = slapdash.random_under_uint_(1234_u16);
    println!("Slapdash number u16 = {}", num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let num = slapdash.random_under_uint_(12345678_u32);
    println!("Slapdash number u32 = {}", num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num = rand.random_under_uint_(1234567890123456_u64);
    println!("Random number u64 = {}", num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num = any.random_under_uint_(12345678901234567890_u128);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let num = slapdash.random_under_uint_(1234_usize);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let num = slapdash.random_under_uint_(12_u8);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let num = slapdash.random_under_uint_(1234_u16);
    println!("Slapdash number u16 = {}", num);

    #[cfg(test)] // It will panic.
    random_should_panic_random_uint_();
    println!("-------------------------------");
}

#[test]
#[should_panic]
fn random_should_panic_random_uint_()
{
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let _num = rand.random_under_uint_(0_u8);
    println!("Random number u8 = {}", _num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let _num = any.random_under_uint_(0_u16);
    println!("Any number u16 = {}", _num);

    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let _num = rand.random_under_uint_(0_u32);
    println!("Random number u32 = {}", _num);

    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let _num = rand.random_under_uint_(0_u64);
    println!("Random number u64 = {}", _num);

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let _num = rand.random_under_uint_(0_u128);
    println!("Random number u128 = {}", _num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let _num = any.random_under_uint_(0_usize);
    println!("Any number usize = {}", _num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let _num = any.random_under_uint_(0_u8);
    println!("Any number u8 = {}", _num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let _num = any.random_under_uint_(0_u16);
    println!("Any number u16 = {}", _num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let _num = any.random_under_uint_(0_u32);
    println!("Any number u32 = {}", _num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let _num = any.random_under_uint_(0_u64);
    println!("Any number u64 = {}", _num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let _num = any.random_under_uint_(0_u128);
    println!("Any number u128 = {}", _num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let _num = slapdash.random_under_uint_(0_usize);
    println!("Slapdash number usize = {}", _num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let _num = slapdash.random_under_uint_(0_u8);
    println!("Slapdash number u8 = {}", _num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let _num = slapdash.random_under_uint_(0_u16);
    println!("Slapdash number u16 = {}", _num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let _num = slapdash.random_under_uint_(0_u32);
    println!("Slapdash number u32 = {}", _num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let _num = rand.random_under_uint_(0_u64);
    println!("Random number u64 = {}", _num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let _num = any.random_under_uint_(0_u128);
    println!("Any number u128 = {}", _num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let _num = slapdash.random_under_uint_(0_usize);
    println!("Slapdash number usize = {}", _num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let _num = slapdash.random_under_uint_(0_u8);
    println!("Slapdash number u8 = {}", _num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let _num = slapdash.random_under_uint_(0_u16);
    println!("Slapdash number u16 = {}", _num);
}

fn random_random_minmax_uint()
{
    println!("random_random_minmax_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    if let Some(num) = rand.random_minmax_uint(12_u8, 21)
        { println!("Random number u8 = {}", num); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any number u16 = {}", num); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    if let Some(num) = rand.random_minmax_uint(12345678_u32, 87654321)
        { println!("Random number u32 = {}", num); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    if let Some(num) = rand.random_minmax_uint(1234567890123456_u64, 6543210987654321)
        { println!("Random number u64 = {}", num); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    if let Some(num) = rand.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
        { println!("Random number u128 = {}", num); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    if let Some(num) = any.random_minmax_uint(123456789_usize, 987654321)
        { println!("Any number usize = {}", num); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    if let Some(num) = any.random_minmax_uint(10, 8_usize)
        { println!("Any number usize = {}", num); }
    else
        { println!("No any unsigned number number greater than or equal to 10 and less than 8!"); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    if let Some(num) = any.random_minmax_uint(12_u8, 21)
        { println!("Any number u8 = {}", num); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
        { println!("Any number u16 = {}", num); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any number u32 = {}", num); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    if let Some(num) = any.random_minmax_uint(1234567890123456_u64, 6543210987654321)
        { println!("Any number u64 = {}", num); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    if let Some(num) = slapdash.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
        { println!("Slapdash number u128 = {}", num); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    if let Some(num) = slapdash.random_minmax_uint(123456789_usize, 987654321)
        { println!("Slapdash number usize = {}", num); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    if let Some(num) = slapdash.random_minmax_uint(10, 8_usize)
        { println!("Slapdash number usize = {}", num); }
    else
        { println!("No slapdash unsigned number number greater than or equal to 10 and less than 8!"); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    if let Some(num) = slapdash.random_minmax_uint(12_u8, 21)
        { println!("Slapdash number u8 = {}", num); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
        { println!("Random number u16 = {}", num); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
        { println!("Any number u32 = {}", num); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    if let Some(num) = slapdash.random_minmax_uint(1234567890123456_u64, 6543210987654321)
        { println!("Slapdash number u64 = {}", num); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    if let Some(num) = slapdash.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
        { println!("Slapdash number u128 = {}", num); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    if let Some(num) = slapdash.random_minmax_uint(10, 8_usize)
        { println!("Slapdash number usize = {}", num); }
    else
        { println!("No slapdash unsigned number number greater than or equal to 10 and less than 8!"); }
    println!("-------------------------------");
}

fn random_random_minmax_uint_()
{
    println!("random_random_minmax_uint_");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let num = rand.random_minmax_uint_(12_u8, 21);
    println!("Random number u8 = {}", num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let num = any.random_minmax_uint_(1234_u16, 6321);
    println!("Random number u16 = {}", num);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let num = rand.random_minmax_uint_(12345678_u32, 87654321);
    println!("Any number u32 = {}", num);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let num = rand.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    println!("Random number u64 = {}", num);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let num = rand.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    println!("Random number u128 = {}", num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num = any.random_minmax_uint_(123456789_usize, 987654321);
    println!("Any number usize = {}", num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let num = any.random_minmax_uint_(12_u8, 21);
    println!("Any number u8 = {}", num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let num = any.random_minmax_uint_(1234_u16, 6321);
    println!("Any number u16 = {}", num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let num = any.random_minmax_uint_(12345678_u32, 87654321);
    println!("Any number u32 = {}", num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let num = any.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    println!("Any number u64 = {}", num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let num = any.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let num = slapdash.random_minmax_uint_(123456789_usize, 987654321);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let num = slapdash.random_minmax_uint_(12_u8, 21);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let num = slapdash.random_minmax_uint_(1234_u16, 6321);
    println!("Slapdash number u16 = {}", num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let num = slapdash.random_minmax_uint_(12345678_u32, 87654321);
    println!("Slapdash number u32 = {}", num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num = rand.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    println!("Random number u64 = {}", num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num = any.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let num = slapdash.random_minmax_uint_(123456789_usize, 987654321);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let num = slapdash.random_minmax_uint_(12_u8, 21);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let num = slapdash.random_minmax_uint_(1234_u16, 6321);
    println!("Slapdash number u16 = {}", num);

    #[cfg(test)] // It will panic.
    random_should_panic_random_minmax_uint_();
    println!("-------------------------------");
}

#[test]
#[should_panic]
fn random_should_panic_random_minmax_uint_()
{
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let _num = rand.random_minmax_uint_(121_u8, 21);
    println!("Random number u8 = {}", _num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let _num = any.random_minmax_uint_(12345_u16, 6321);
    println!("Any number u16 = {}", _num);

    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let _num = rand.random_minmax_uint_(123456789_u32, 87654321);
    println!("Random number u32 = {}", _num);

    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let _num = rand.random_minmax_uint_(12345678901234567_u64, 6543210987654321);
    println!("Random number u64 = {}", _num);

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let _num = rand.random_minmax_uint_(123456789012345678901_u128, 19876543210987654321);
    println!("Random number u128 = {}", _num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num = any.random_minmax_uint_(1234567890_usize, 987654321);
    println!("Any number usize = {}", num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let num = any.random_minmax_uint_(123_u8, 21);
    println!("Any number u8 = {}", num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let num = any.random_minmax_uint_(12345_u16, 6321);
    println!("Any number u16 = {}", num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let num = any.random_minmax_uint_(123456789_u32, 87654321);
    println!("Any number u32 = {}", num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let num = any.random_minmax_uint_(12345678901234567_u64, 6543210987654321);
    println!("Any number u64 = {}", num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let num = any.random_minmax_uint_(123456789012345678901_u128, 19876543210987654321);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let num = slapdash.random_minmax_uint_(1234567890_usize, 987654321);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let num = slapdash.random_minmax_uint_(123_u8, 21);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let num = any.random_minmax_uint_(12345_u16, 6321);
    println!("Slapdash number u16 = {}", num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let num = slapdash.random_minmax_uint_(123456789_u32, 87654321);
    println!("Slapdash number u32 = {}", num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num = rand.random_minmax_uint_(12345678901234567_u64, 6543210987654321);
    println!("Random number u64 = {}", num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num = any.random_minmax_uint_(123456789012345678901_u128, 19876543210987654321);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let num = slapdash.random_minmax_uint_(1234567890_usize, 987654321);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let num = slapdash.random_minmax_uint_(123_u8, 21);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let num = slapdash.random_minmax_uint_(12345_u16, 6321);
    println!("Slapdash number u16 = {}", num);
}

fn random_random_odd_uint()
{
    println!("random_random_odd_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    println!("Any odd number usize = {}", any.random_odd_uint::<usize>());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_odd_under_uint()
{
    println!("random_random_odd_under_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(0_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned number number under 0!"); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 1!"); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(0_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned odd number under 0!"); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned odd number under 1!"); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(0_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned odd number under 0!"); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 1!"); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(0_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(0_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(0_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 0!"); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(0_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 0!"); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 1!"); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(0_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 0!"); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    if let Some(num) = rand.random_odd_under_uint(12_u8)
        { println!("Random odd number u8 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234_u16)
        { println!("Random odd number u16 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678_u32)
        { println!("Random odd number u32 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
        { println!("Random odd number u64 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
        { println!("Random odd number u128 = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(123456789_usize)
        { println!("Random odd number usize = {}", num); }
    if let Some(num) = rand.random_odd_under_uint(1_usize)
        { println!("Random odd number usize = {}", num); }
    else
        { println!("No random unsigned odd number under 1!"); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    if let Some(num) = any.random_odd_under_uint(12_u8)
        { println!("Any odd number u8 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234_u16)
        { println!("Any odd number u16 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678_u32)
        { println!("Any odd number u32 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
        { println!("Any odd number u64 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
        { println!("Any odd number u128 = {}", num); }
    if let Some(num) = any.random_odd_under_uint(123456789_usize)
        { println!("Any odd number usize = {}", num); }
    if let Some(num) = any.random_odd_under_uint(0_usize)
        { println!("Any odd number usize = {}", num); }
    else
        { println!("No any unsigned odd number under 0!"); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 1!"); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(0_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 0!"); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    if let Some(num) = slapdash.random_odd_under_uint(12_u8)
        { println!("Slapdash odd number u8 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
        { println!("Slapdash odd number u16 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
        { println!("Slapdash odd number u32 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
        { println!("Slapdash odd number u64 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
        { println!("Slapdash odd number u128 = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
        { println!("Slapdash odd number usize = {}", num); }
    if let Some(num) = slapdash.random_odd_under_uint(1_usize)
        { println!("Slapdash odd number usize = {}", num); }
    else
        { println!("No slapdash unsigned odd number under 1!"); }
    println!("-------------------------------");
}

fn random_random_odd_under_uint_()
{
    println!("random_random_odd_under_uint_");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_(123456789_usize);
    println!("Random odd number usize = {}", num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_(123456789_usize);
    println!("Random odd number usize = {}", num);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_(123456789_usize);
    println!("Random odd number usize = {}", num);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_(123456789_usize);
    println!("Random odd number usize = {}", num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();

    let num = rand.random_odd_under_uint_(12_u8);
    println!("Random odd number u8 = {}", num);

    let num = rand.random_odd_under_uint_(1234_u16);
    println!("Random odd number u16 = {}", num);

    let num = rand.random_odd_under_uint_(12345678_u32);
    println!("Random odd number u32 = {}", num);

    let num = rand.random_odd_under_uint_(1234567890123456_u64);
    println!("Random odd number u64 = {}", num);

    let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    println!("Random odd number u128 = {}", num);

    let num = rand.random_odd_under_uint_(123456789_usize);
    println!("Random odd number usize = {}", num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();

    let num = any.random_odd_under_uint_(12_u8);
    println!("Any odd number u8 = {}", num);

    let num = any.random_odd_under_uint_(1234_u16);
    println!("Any odd number u16 = {}", num);

    let num = any.random_odd_under_uint_(12345678_u32);
    println!("Any odd number u32 = {}", num);

    let num = any.random_odd_under_uint_(1234567890123456_u64);
    println!("Any odd number u64 = {}", num);

    let num = any.random_odd_under_uint_(12345678901234567890_u128);
    println!("Any odd number u128 = {}", num);

    let num = any.random_odd_under_uint_(123456789_usize);
    println!("Any odd number usize = {}", num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();

    let num = slapdash.random_odd_under_uint_(12_u8);
    println!("Slapdash odd number u8 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234_u16);
    println!("Slapdash odd number u16 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678_u32);
    println!("Slapdash odd number u32 = {}", num);

    let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    println!("Slapdash odd number u64 = {}", num);

    let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    println!("Slapdash odd number u128 = {}", num);

    let num = slapdash.random_odd_under_uint_(123456789_usize);
    println!("Slapdash odd number usize = {}", num);

    #[cfg(test)] // It will panic.
    random_should_panic_random_odd_under_uint_();
    println!("-------------------------------");
}

#[test]
#[should_panic]
fn random_should_panic_random_odd_under_uint_()
{
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let _num = rand.random_odd_under_uint_(0_u8);
    println!("Random number u8 = {}", _num);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let _num = any.random_odd_under_uint_(1_u16);
    println!("Any number u16 = {}", _num);

    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let _num = rand.random_odd_under_uint_(0_u32);
    println!("Random number u32 = {}", _num);

    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let _num = rand.random_odd_under_uint_(1_u64);
    println!("Random number u64 = {}", _num);

    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let _num = rand.random_odd_under_uint_(0_u128);
    println!("Random number u128 = {}", _num);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num = any.random_odd_under_uint_(1_usize);
    println!("Any number usize = {}", num);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let num = any.random_odd_under_uint_(0_u8);
    println!("Any number u8 = {}", num);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let num = any.random_odd_under_uint_(1_u16);
    println!("Any number u16 = {}", num);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let num = any.random_odd_under_uint_(0_u32);
    println!("Any number u32 = {}", num);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let num = any.random_odd_under_uint_(1_u64);
    println!("Any number u64 = {}", num);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let num = any.random_odd_under_uint_(0_u128);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let num = slapdash.random_odd_under_uint_(1_usize);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let num = slapdash.random_odd_under_uint_(0_u8);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let num = any.random_odd_under_uint_(1_u16);
    println!("Slapdash number u16 = {}", num);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let num = slapdash.random_odd_under_uint_(0_u32);
    println!("Slapdash number u32 = {}", num);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num = rand.random_odd_under_uint_(1_u64);
    println!("Random number u64 = {}", num);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num = any.random_odd_under_uint_(0_u128);
    println!("Any number u128 = {}", num);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let num = slapdash.random_odd_under_uint_(1_usize);
    println!("Slapdash number usize = {}", num);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let num = slapdash.random_odd_under_uint_(0_u8);
    println!("Slapdash number u8 = {}", num);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let num = slapdash.random_odd_under_uint_(1_u16);
    println!("Slapdash number u16 = {}", num);
}

fn random_random_with_msb_set_uint()
{
    println!("random_random_with_msb_set_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_odd_with_msb_set_uint()
{
    println!("random_random_odd_with_msb_set_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    println!("-------------------------------");
}

fn random_random_prime_using_miller_rabin_uint()
{
    println!("random_random_prime_using_miller_rabin_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    println!("-------------------------------");
}

fn random_random_prime_with_msb_set_using_miller_rabin_uint()
{
    println!("random_random_prime_with_msb_set_using_miller_rabin_uint");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    println!("-------------------------------");
}

fn random_random_array()
{
    println!("random_random_array");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let num: [u128; 5] = rand.random_array();
    for i in 0..5
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let num: [u64; 10] = any.random_array();
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let num: [u32; 16] = rand.random_array();
    for i in 0..16
        { println!("Random number {} => {}", i, num[i]); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let num: [u16; 20] = rand.random_array();
    for i in 0..20
        { println!("Random number {} => {}", i, num[i]); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let num: [u8; 32] = rand.random_array();
    for i in 0..32
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let num: [usize; 10] = any.random_array();
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let num: [u128; 4] = any.random_array();
    for i in 0..4
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let num: [u64; 10] = any.random_array();
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let num: [u32; 16] = any.random_array();
    for i in 0..16
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let num: [u16; 10] = any.random_array();
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let num: [u8; 8] = any.random_array();
    for i in 0..8
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let num: [usize; 16] = slapdash.random_array();
    for i in 0..16
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let num: [u128; 16] = slapdash.random_array();
    for i in 0..16
        { println!("slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let num: [u64; 8] = slapdash.random_array();
    for i in 0..8
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let num: [u32; 4] = slapdash.random_array();
    for i in 0..4
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let num: [u16; 5] = rand.random_array();
    for i in 0..5
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let num: [u8; 10] = any.random_array();
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let num: [u128; 4] = slapdash.random_array();
    for i in 0..4
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let num: [u64; 16] = slapdash.random_array();
    for i in 0..16
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let num: [u32; 8] = slapdash.random_array();
    for i in 0..8
        { println!("Slapdash number {} => {}", i, num[i]); }
    println!("-------------------------------");
}

fn random_put_random_in_array()
{
    println!("random_put_random_in_array");
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let mut num = [0_u128; 5];
    rand.put_random_in_array(&mut num);
    for i in 0..5
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let mut num = [0_u64; 10];
    any.put_random_in_array(&mut num);
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let mut num = [0_u32; 16];
    rand.put_random_in_array(&mut num);
    for i in 0..16
        { println!("Random number {} => {}", i, num[i]); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let mut num = [0_u16; 20];
    rand.put_random_in_array(&mut num);
    for i in 0..20
        { println!("Random number {} => {}", i, num[i]); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let mut num = [0_u8; 32];
    rand.put_random_in_array(&mut num);
    for i in 0..32
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let mut num = [0_usize; 10];
    any.put_random_in_array(&mut num);
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let mut num = [0_u128; 4];
    any.put_random_in_array(&mut num);
    for i in 0..4
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let mut num = [0_u64; 10];
    any.put_random_in_array(&mut num);
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let mut num = [0_u32; 16];
    any.put_random_in_array(&mut num);
    for i in 0..16
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let mut num = [0_u16; 10];
    any.put_random_in_array(&mut num);
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let mut num = [0_u8; 8];
    any.put_random_in_array(&mut num);
    for i in 0..8
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let mut num = [0_usize; 16];
    slapdash.put_random_in_array(&mut num);
    for i in 0..16
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let mut num = [0_u128; 16];
    slapdash.put_random_in_array(&mut num);
    for i in 0..16
        { println!("slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let mut num = [0_u64; 8];
    slapdash.put_random_in_array(&mut num);
    for i in 0..8
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let mut num = [0_u32; 4];
    slapdash.put_random_in_array(&mut num);
    for i in 0..4
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let mut num = [0_u16; 5];
    rand.put_random_in_array(&mut num);
    for i in 0..5
        { println!("Random number {} => {}", i, num[i]); }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let mut num = [0_u8; 10];
    any.put_random_in_array(&mut num);
    for i in 0..10
        { println!("Any number {} => {}", i, num[i]); }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let mut num = [0_u128; 4];
    slapdash.put_random_in_array(&mut num);
    for i in 0..4
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let mut num = [0_u64; 16];
    slapdash.put_random_in_array(&mut num);
    for i in 0..16
        { println!("Slapdash number {} => {}", i, num[i]); }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let mut num = [0_u32; 8];
    slapdash.put_random_in_array(&mut num);
    for i in 0..8
        { println!("Slapdash number {} => {}", i, num[i]); }
    println!("-------------------------------");
}
//////////////////
fn random_random_biguint()
{
    println!("random_random_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_MD5;

    define_utypes_with!(u128);
    let mut rand = Slapdash_MD5::new();
    let biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);
    println!("-------------------------------");
}

fn random_random_under_biguint()
{
    println!("random_random_under_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_SHA0;

    define_utypes_with!(u64);
    let mut rand = Slapdash_SHA0::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    println!("-------------------------------");
}

fn random_random_under_biguint_()
{
    println!("random_random_under_biguint_");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_SHA1;

    define_utypes_with!(u32);
    let mut rand = Slapdash_SHA1::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_odd_biguint()
{
    println!("random_random_odd_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA2_256;

    define_utypes_with!(u16);
    let mut rand = Any_SHA2_256::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_odd_under_biguint()
{
    println!("random_random_odd_under_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_MD4;

    define_utypes_with!(u128);
    let mut rand = Slapdash_MD4::new();
    let ceiling = U1024::max().wrapping_div_uint(4_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    println!("-------------------------------");
}

fn random_random_odd_under_biguint_()
{
    println!("random_random_odd_under_biguint_");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any_SHA2_512;

    define_utypes_with!(u8);
    let mut rand = Any_SHA2_512::new();
    let ceiling = U1024::max().wrapping_div_uint(3_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_with_msb_set_biguint()
{
    println!("random_random_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_MD5;

    define_utypes_with!(u64);
    let mut rand = Slapdash_MD5::new();
    let biguint: U512 = rand.random_with_msb_set_biguint();
    println!("Random Number: {}", biguint);
    println!("-------------------------------");
}

fn random_random_odd_with_msb_set_biguint()
{
    println!("random_random_odd_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Any;

    define_utypes_with!(u32);
    let mut rand = Any::new();
    let num:U512 = rand.random_odd_with_msb_set_biguint();
    println!("512-bit Random Odd Number = {}", num);
    assert!(num > U512::halfmax());
    assert!(num.is_odd());
    println!("-------------------------------");
}

fn random_random_prime_using_miller_rabin_biguint()
{
    println!("random_random_prime_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;

    define_utypes_with!(u16);
    let mut rand = Random::new();
    let num:U512 = rand.random_prime_using_miller_rabin_biguint(5);
    println!("Random Prime Number = {}", num);
    assert!(num.is_odd());
    println!("-------------------------------");
}

fn random_random_prime_with_msb_set_using_miller_rabin_biguint()
{
    println!("random_random_prime_with_msb_set_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    use cryptocol::random::Slapdash_SHA1;

    define_utypes_with!(u16);
    let mut rand = Slapdash_SHA1::new();
    let num:U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("512-bit Random Prime Number = {}", num);
    assert!(num.is_odd());
    println!("-------------------------------");
}


/*
use std::ops::*;
use std::fmt::{ Display, Debug };
use rand::{ rngs, RngCore };

use cryptocol::number::SmallUInt;
use cryptocol::random::{ random_Engine, random_Generic };

pub struct OsRng;
{
    hash_code: [u64; 8]
}

impl random_Engine for OsRng
{
    #[inline]
    fn new() -> Self
    {
        Self { hash_code: [0_u64; 8] }
    }

    #[inline]
    fn new_with<T, const N: usize>(_: &[T; N]) -> Self
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
    {
        Self::new()
    }

    #[inline]
    fn sow_array<T, const N: usize>(&mut self, _: &[T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
    {}

    #[inline]
    fn harvest(&mut self, _: bool) -> [u64; 8]
    {
        [rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
        rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
        rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
        rngs::OsRng.next_u64(), rngs::OsRng.next_u64()]
    }
}

fn Hash_OS_Rng_main()
{
    pub type random_OsRng = random_Generic<OsRng>;
    
    let mut r = random_OsRng::new();
    println!("random_OsRng u8 = {}", r.random_u8());
    println!("random_OsRng u16 = {}", r.random_u16());
    println!("random_OsRng u32 = {}", r.random_u32());
    println!("random_OsRng u64 = {}", r.random_u64());
    println!("random_OsRng u128 = {}", r.random_u128());
    println!("random_OsRng under 123456789 = {}", r.random_under_uint_(123456789_u64));
    println!("random_OsRng prime number = {}", r.random_prime_using_miller_rabin_uint::<u128>(5));
    println!("random_OsRng under BigUInt = {}", r.random_biguint::<u64, 8>());
    println!("random_OsRng odd BigUInt = {}", r.random_odd_biguint::<u64, 8>());
    println!("random_OsRng BigUInt prime number = {}", r.random_prime_using_miller_rabin_biguint::<u64, 8>(5));
}
*/
