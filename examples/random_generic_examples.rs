// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


// use cryptocol::concurrency::do_simultaneously_unit as do_simultaneously;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;
use std::thread::{ spawn, available_parallelism };

fn do_simultaneously(jobs: Vec<fn()>)
{
    let number_of_threads: usize = match available_parallelism()
    {
        Ok(non_zero) => non_zero.get() as usize,
        Err(_) => 1_usize,
    };
    
    if number_of_threads == 1
    {
        for work in jobs
            { work(); }
        return;
    }

    let mut threads = Vec::new();
    let (tx, rx) = channel::<fn()>();
    let receiver = Arc::new(Mutex::new(rx));
    for _ in 0..number_of_threads
    {
        let rxx = receiver.clone();
        threads.push(spawn(move ||
        {
            loop
            {
                let r = rxx.lock().unwrap();
                match r.recv()
                {
                    Ok(work) => { drop(r); work(); },
                    _ => { return },
                }
            }
        }));
    }

    for job in jobs
        { tx.clone().send(job).unwrap(); }
    drop(tx);
    for thread in threads
        { thread.join().unwrap(); }
}

pub fn main()
{
    random_quick_start();
    random_constructors();
    random_smalluint();
    random_biguint();
    random_prepare_primes();
    find_u256_primes();
    find_u512_primes();
    find_u1024_primes();
    find_u2048_primes();
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

fn random_constructors()
{
    random_new();
    random_new_with();
    random_new_with_generators_seeds();
    random_new_with_generators_seed_arrays();
    random_new_with_generators_seed_collector();
    random_new_with_generators_seed_collector_seeds();
    random_new_with_generators_seed_collector_seed_arrays();
    random_get_seed_collector();
    random_set_seed_collector();
    random_reset_seed_collector();
}

fn random_new()
{
    println!("random_new");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut thread = Vec::<fn()>::new();

    // Example for Random
    thread.push(||{
        use cryptocol::random::Random;
        let mut rand = Random::new();
        let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number = {}", num);
    });
    
    // Example for Any
    thread.push(|| {
        use cryptocol::random::Any;
        let mut any = Any::new();
        let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number = {}", num);
    });
    
    // Example for Random_BIG_KECCAK_1024
    thread.push(|| {
        use cryptocol::random::Random_BIG_KECCAK_1024;
        let mut rand = Random_BIG_KECCAK_1024::new();
        let num: U1024 = rand.random_with_msb_set_biguint();
        println!("Random number = {}", num);
    });
    
    // Example for Random_SHA3_512
    thread.push(|| {
        use cryptocol::random::Random_SHA3_512;
        let mut rand = Random_SHA3_512::new();
        let num: U768 = rand.random_odd_biguint();
        println!("Random number = {}", num);
    });
    
    // Example for Random_SHA2_512
    thread.push(|| {
        use cryptocol::random::Random_SHA2_512;
        let mut rand = Random_SHA2_512::new();
        let num: U512 = rand.random_biguint();
        println!("Random number = {}", num);
    });

    // Example for Any_SHAKE_256
    thread.push(|| {
        use cryptocol::random::Any_SHAKE_256;
        let mut any = Any_SHAKE_256::new();
        let num: U384 = any.random_biguint();
        println!("Any number = {}", num);
    });
    
    // Example for Any_SHAKE_128
    thread.push(|| {
        use cryptocol::random::Any_SHAKE_128;
        let mut any = Any_SHAKE_128::new();
        println!("Any number = {}", any.random_u128());
    });
    
    // Example for Any_SHA3_512
    thread.push(|| {
        use cryptocol::random::Any_SHA3_512;
        let mut any = Any_SHA3_512::new();
        println!("Any number = {}", any.random_u64());
    });
    
    // Example for Any_SHA3_256
    thread.push(|| {
        use cryptocol::random::Any_SHA3_256;
        let mut any = Any_SHA3_256::new();
        println!("Any number = {}", any.random_u32());
    });
    
    // Example for Any_SHA2_512
    thread.push(|| {
        use cryptocol::random::Any_SHA2_512;
        let mut any = Any_SHA2_512::new();
        println!("Any number = {}", any.random_u16());
    });

    // Example for Any_SHA2_256
    thread.push(|| {
        use cryptocol::random::Any_SHA2_256;
        let mut any = Any_SHA2_256::new();
        println!("Any number = {}", any.random_u8());
    });
    
    // Example for Slapdash_SHA1
    thread.push(|| {
        use cryptocol::random::Slapdash_SHA1;
        let mut slapdash = Slapdash_SHA1::new();
        println!("Slapdash number = {}", slapdash.random_usize());
    });
    
    // Example for Slapdash_SHA0
    thread.push(|| {
        use cryptocol::random::Slapdash_SHA0;
        let mut slapdash = Slapdash_SHA0::new();
        println!("Slapdash number = {}", slapdash.random_u64());
    });
    
    // Example for Slapdash_MD5
    thread.push(|| {
        use cryptocol::random::Slapdash_MD5;
        let mut slapdash = Slapdash_MD5::new();
        println!("Slapdash number = {}", slapdash.random_u32());
    });
    
    // Example for Slapdash_MD4
    thread.push(|| {
        use cryptocol::random::Slapdash_MD4;
        let mut slapdash = Slapdash_MD4::new();
        println!("Slapdash number = {}", slapdash.random_u16());
    });
    
    // Example for Random_Rijndael
    thread.push(|| {
        use cryptocol::random::Random_Rijndael;
        let mut rand = Random_Rijndael::new();
        let num: U512 = rand.random_with_msb_set_biguint();
        println!("Random number = {}", num);
    });
    
    // Example for Any_Rijndael
    thread.push(|| {
        use cryptocol::random::Any_Rijndael;
        let mut any = Any_Rijndael::new();
        let num: U384 = any.random_biguint();
        println!("Any number = {}", num);
    });
    
    // Example for Slapdash_DES
    thread.push(|| {
        use cryptocol::random::Slapdash_DES;
        let mut slapdash = Slapdash_DES::new();
        let num: U256 = slapdash.random_odd_biguint();
        println!("Slapdash number = {}", num);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_new_with()
{
    println!("random_new_with");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut thread = Vec::<fn()>::new();

    // Example for BIG_KECCAK_1024
    thread.push(|| {
        use cryptocol::hash::BIG_KECCAK_1024;
        let mut rand = RandGen::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new());
        let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random number = {}", num);
    });

    // Example for SHA3_512
    thread.push(|| {
        use cryptocol::hash::SHA3_512;
        let mut any = AnyGen::new_with(SHA3_512::new(), SHA3_512::new());
        let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any number = {}", num);
    });

    // Example for SHA2_512
    thread.push(|| {
        use cryptocol::hash::SHA2_512;
        let mut any = AnyGen::new_with(SHA2_512::new(), SHA2_512::new());
        let num: U1024 = any.random_with_msb_set_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHAKE_256
    thread.push(|| {
        use cryptocol::hash::SHAKE_256;
        let mut rand = RandGen::new_with(SHAKE_256::new(), SHAKE_256::new());
        let num: U768 = rand.random_odd_biguint();
        println!("Random number = {}", num);
    });

    // Example for SHAKE_128
    thread.push(|| {
        use cryptocol::hash::SHAKE_128;
        let mut any = AnyGen::new_with(SHAKE_128::new(), SHAKE_128::new());
        let num: U512 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA3_256
    thread.push(|| {
        use cryptocol::hash::SHA3_256;
        let mut any = AnyGen::new_with(SHA3_256::new(), SHA3_256::new());
        let num: U384 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA2_256
    thread.push(|| {
        use cryptocol::hash::SHA2_256;
        let mut any = AnyGen::new_with(SHA2_256::new(), SHA2_256::new());
        println!("Any number = {}", any.random_u128());
    });

    // Example for SHA1 and SHA0
    thread.push(|| {
        use cryptocol::hash::{ SHA1, SHA0 };
        let mut any = SlapdashGen::new_with(SHA1::new(), SHA0::new());
        println!("Any number = {}", any.random_u64());
    });

    // Example for MD5 and MD4
    thread.push(|| {
        use cryptocol::hash::{ MD5, MD4 };
        let mut any = SlapdashGen::new_with(MD5::new(), MD4::new());
        println!("Any number = {}", any.random_u32());
    });

    // Example for AES_128
    thread.push(|| {
        use cryptocol::symmetric::AES_128;
        let mut rand = RandGen::new_with(AES_128::new(), AES_128::new());
        println!("Random number = {}", rand.random_u16());
    });

    // Example for DES
    thread.push(|| {
        use cryptocol::symmetric::DES;
        let mut slapdash = SlapdashGen::new_with(DES::new(), DES::new());
        println!("Slapdash number = {}", slapdash.random_u8());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_new_with_generators_seeds()
{
    println!("random_new_with_generators_seeds");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut thread = Vec::<fn()>::new();

    // Example for BIG_KECCAK_1024
    thread.push(|| {
        use cryptocol::hash::BIG_KECCAK_1024;
        let mut rand = RandGen::new_with_generators_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), 10500872879054459758_u64, 15887751380961987625_u64);
        let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random number = {}", num);
    });

    // Example for SHA3_512
    thread.push(|| {
        use cryptocol::hash::SHA3_512;
        let mut any = AnyGen::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), 100, 25);
        let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any number = {}", num);
    });

    // Example for SHA2_512
    thread.push(|| {
        use cryptocol::hash::SHA2_512;
        let mut any = AnyGen::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), 0, 0);
        let num: U1024 = any.random_with_msb_set_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHAKE_256
    thread.push(|| {
        use cryptocol::hash::SHAKE_256;
        let mut rand = RandGen::new_with_generators_seeds(SHAKE_256::new(), SHAKE_256::new(), u64::MAX, u64::MAX);
        let num: U768 = rand.random_odd_biguint();
        println!("Random number = {}", num);
    });

    // Example for SHAKE_128
    thread.push(|| {
        use cryptocol::hash::SHAKE_128;
        let mut any = AnyGen::new_with_generators_seeds(SHAKE_128::new(), SHAKE_128::new(), 123456789, 987654321);
        let num: U512 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA3_256
    thread.push(|| {
        use cryptocol::hash::SHA3_256;
        let mut any = AnyGen::new_with_generators_seeds(SHA3_256::new(), SHA3_256::new(), u32::MAX as u64, u32::MAX as u64);
        let num: U384 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA2_256
    thread.push(|| {
        use cryptocol::hash::SHA2_256;
        let mut any = AnyGen::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), 15698731215687456325, 10684237915728469725);
        println!("Any number = {}", any.random_u128());
    });

    // Example for SHA1 and SHA0
    thread.push(|| {
        use cryptocol::hash::{ SHA1, SHA0 };
        let mut slapdash = SlapdashGen::new_with_generators_seeds(SHA1::new(), SHA0::new(), 2879054410500759758, 15887876257513809619);
        println!("Slapdash number = {}", slapdash.random_u64());
    });

    // Example for MD5 and MD4
    thread.push(|| {
        use cryptocol::hash::{ MD5, MD4 };
        let mut slapdash = SlapdashGen::new_with_generators_seeds(MD5::new(), MD4::new(), 610458805, 215793685);
        println!("Slapdash number = {}", slapdash.random_u32());
    });

    // Example for AES_128
    thread.push(|| {
        use cryptocol::symmetric::AES_128;
        let mut rand = RandGen::new_with_generators_seeds(AES_128::new(), AES_128::new(), 18782, 50558);
        println!("Random number = {}", rand.random_u16());
    });

    // Example for DES
    thread.push(|| {
        use cryptocol::symmetric::DES;
        let mut slapdash = SlapdashGen::new_with_generators_seeds(DES::new(), DES::new(), 0, 125);
        println!("Slapdash number = {}", slapdash.random_u8());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}
fn random_new_with_generators_seed_arrays()
{
    println!("random_new_with_generators_seed_arrays");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut thread = Vec::<fn()>::new();

    // Example for BIG_KECCAK_1024
    thread.push(|| {
        use cryptocol::hash::BIG_KECCAK_1024;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut rand = RandGen::new_with_generators_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed, aux);
        let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random number = {}", num);
    });

    // Example for SHA3_512
    thread.push(|| {
        use cryptocol::hash::SHA3_512;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed, aux);
        let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any number = {}", num);
    });

    // Example for SHA2_512
    thread.push(|| {
        use cryptocol::hash::SHA2_512;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux);
        let num: U1024 = any.random_with_msb_set_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHAKE_256
    thread.push(|| {
        use cryptocol::hash::SHAKE_256;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut rand = RandGen::new_with_generators_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed, aux);
        let num: U768 = rand.random_odd_biguint();
        println!("Random number = {}", num);
    });

    // Example for SHAKE_128
    thread.push(|| {
        use cryptocol::hash::SHAKE_128;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut any = AnyGen::new_with_generators_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed, aux);
        let num: U512 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA3_256
    thread.push(|| {
        use cryptocol::hash::SHA3_256;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed, aux);
        let num: U384 = any.random_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHA2_256
    thread.push(|| {
        use cryptocol::hash::SHA2_256;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed, aux);
        println!("Any number = {}", any.random_u128());
    });

    // Example for SHA1 and SHA0
    thread.push(|| {
        use cryptocol::hash::{ SHA1, SHA0 };
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(SHA1::new(), SHA0::new(), seed, aux);
        println!("Slapdash number = {}", slapdash.random_u64());
    });

    // Example for MD5 and MD4
    thread.push(|| {
        use cryptocol::hash::{ MD5, MD4 };
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(MD5::new(), MD4::new(), seed, aux);
        println!("Slapdash number = {}", slapdash.random_u32());
    });

    // Example for AES_128
    thread.push(|| {
        use cryptocol::symmetric::AES_128;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut rand = RandGen::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux);
        println!("Random number = {}", rand.random_u16());
    });

    // Example for DES
    thread.push(|| {
        use cryptocol::symmetric::DES;
        let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
        let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
        let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(DES::new(), DES::new(), seed, aux);
        println!("Slapdash number = {}", slapdash.random_u8());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector()
{
    println!("random_new_with_generators_seed_collector");
    use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut thread = Vec::<fn()>::new();

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
    thread.push(|| {
        use cryptocol::hash::BIG_KECCAK_1024;
        let mut rand = RandGen::new_with_generators_seed_collector(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector);
        let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random number = {}", num);
    });

    // Example for SHA3_512
    thread.push(|| {
        use cryptocol::hash::SHA3_512;
        let mut any = AnyGen::new_with_generators_seed_collector(SHA3_512::new(), SHA3_512::new(), seed_collector);
        let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any number = {}", num);
    });

    // Example for SHA2_512
    thread.push(|| {
        use cryptocol::hash::SHA2_512;
        let mut any = AnyGen::new_with_generators_seed_collector(SHA2_512::new(), SHA2_512::new(),seed_collector);
        let num: U1024 = any.random_with_msb_set_biguint();
        println!("Any number = {}", num);
    });

    // Example for SHAKE_256
    thread.push(|| {
        use cryptocol::hash::SHAKE_256;
        let mut rand = RandGen::new_with_generators_seed_collector(SHAKE_256::new(), SHAKE_256::new(), seed_collector);
        let num: U768 = rand.random_odd_biguint();
        println!("Random number = {}", num);
    });

    // Example for SHAKE_128
    thread.push(|| {
        use cryptocol::hash::SHAKE_128;
        let mut any = AnyGen::new_with_generators_seed_collector(SHAKE_128::new(), SHAKE_128::new(), seed_collector);
        let num: U512 = any.random_biguint();
    println!("Any number = {}", num);
    });

    // Example for SHA3_256
    thread.push(|| {
        use cryptocol::hash::SHA3_256;
        let mut any = AnyGen::new_with_generators_seed_collector(SHA3_256::new(), SHA3_256::new(), seed_collector);
        let num: U384 = any.random_biguint();
    println!("Any number = {}", num);
    });

    // Example for SHA2_256
    thread.push(|| {
        use cryptocol::hash::SHA2_256;
        let mut any = AnyGen::new_with_generators_seed_collector(SHA2_256::new(), SHA2_256::new(), seed_collector);
        println!("Any number = {}", any.random_u128());
    });

    // Example for SHA1 and SHA0
    thread.push(|| {
        use cryptocol::hash::{ SHA1, SHA0 };
        let mut slapdash = SlapdashGen::new_with_generators_seed_collector(SHA1::new(), SHA0::new(), seed_collector);
        println!("Slapdash number = {}", slapdash.random_u64());
    });

    // Example for MD5 and MD4
    thread.push(|| {
        use cryptocol::hash::{ MD5, MD4 };
        let mut slapdash = SlapdashGen::new_with_generators_seed_collector(MD5::new(), MD4::new(), seed_collector);
        println!("Slapdash number = {}", slapdash.random_u32());
    });

    // Example for AES_128
    thread.push(|| {
        use cryptocol::symmetric::AES_128;
        let mut rand = RandGen::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector);
        println!("Random number = {}", rand.random_u16());
    });

    // Example for DES
    thread.push(|| {
        use cryptocol::symmetric::DES;
        let mut slapdash = SlapdashGen::new_with_generators_seed_collector(DES::new(), DES::new(), seed_collector);
        println!("Slapdash number = {}", slapdash.random_u8());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector_seeds()
{
    println!("random_new_with_generators_seed_collector_seeds");
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
    let mut rand = RandGen::new_with_generators_seed_collector_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector, 10500872879054459758_u64, 15887751380961987625_u64);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA3_512::new(), SHA3_512::new(), seed_collector, 100, 25);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA2_512::new(), SHA2_512::new(), seed_collector, 0, 0);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut rand = RandGen::new_with_generators_seed_collector_seeds(SHAKE_256::new(), SHAKE_256::new(), seed_collector, u64::MAX, u64::MAX);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHAKE_128::new(), SHAKE_128::new(), seed_collector, 123456789, 987654321);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA3_256::new(), SHA3_256::new(), seed_collector, u32::MAX as u64, u32::MAX as u64);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA2_256::new(), SHA2_256::new(), seed_collector, 15698731215687456325, 10684237915728469725);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(SHA1::new(), SHA0::new(), seed_collector, 2879054410500759758, 15887876257513809619);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(MD5::new(), MD4::new(), seed_collector, 610458805, 215793685);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let mut rand = RandGen::new_with_generators_seed_collector_seeds(AES_128::new(), AES_128::new(), seed_collector, 18782, 50558);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(DES::new(), DES::new(), seed_collector, 0, 125);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_new_with_generators_seed_collector_seed_arrays()
{
    println!("random_new_with_generators_seed_collector_seed_arrays");
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
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector, seed, aux);
    let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    println!("Random number = {}", num);

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed_collector, seed, aux);
    let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    println!("Any number = {}", num);

    // Example for SHA2_512
    use cryptocol::hash::SHA2_512;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux);
    let num: U1024 = any.random_with_msb_set_biguint();
    println!("Any number = {}", num);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed_collector, seed, aux);
    let num: U768 = rand.random_odd_biguint();
    println!("Random number = {}", num);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed_collector, seed, aux);
    let num: U512 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed_collector, seed, aux);
    let num: U384 = any.random_biguint();
    println!("Any number = {}", num);

    // Example for SHA2_256
    use cryptocol::hash::SHA2_256;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed_collector, seed, aux);
    println!("Any number = {}", any.random_u128());

    // Example for SHA1 and SHA0
    use cryptocol::hash::{ SHA1, SHA0 };
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(SHA1::new(), SHA0::new(), seed_collector, seed, aux);
    println!("Slapdash number = {}", slapdash.random_u64());

    // Example for MD5 and MD4
    use cryptocol::hash::{ MD5, MD4 };
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(MD5::new(), MD4::new(), seed_collector, seed, aux);
    println!("Slapdash number = {}", slapdash.random_u32());

    // Example for AES_128
    use cryptocol::symmetric::AES_128;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(AES_128::new(), AES_128::new(), seed_collector, seed, aux);
    println!("Random number = {}", rand.random_u16());

    // Example for DES
    use cryptocol::symmetric::DES;
    let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(DES::new(), DES::new(), seed_collector, seed, aux);
    println!("Slapdash number = {}", slapdash.random_u8());
    println!("-------------------------------");
}

fn random_get_seed_collector()
{
    println!("random_get_seed_collector");
    // Example for Random
    use cryptocol::random::Random;
    let rand = Random::new();
    let seed = rand.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any
    use cryptocol::random::Any;
    let any = Any::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let rand = Random_BIG_KECCAK_1024::new();
    let seed = rand.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let rand = Random_SHA3_512::new();
    let seed = rand.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let rand = Random_SHA2_512::new();
    let seed = rand.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let any = Any_SHAKE_256::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let any = Any_SHAKE_128::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let any = Any_SHA3_512::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let any = Any_SHA3_256::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let any = Any_SHA2_512::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let any = Any_SHA2_256::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let slapdash = Slapdash_SHA1::new();
    let seed = slapdash.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let slapdash = Slapdash_SHA0::new();
    let seed = slapdash.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let slapdash = Slapdash_MD5::new();
    let seed = slapdash.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let slapdash = Slapdash_MD4::new();
    let seed = slapdash.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let rand = Random_Rijndael::new();
    let seed = rand.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let any = Any_Rijndael::new();
    let seed = any.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    
    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let slapdash = Slapdash_DES::new();
    let seed = slapdash.get_seed_collector()();
    print!("seed = ");
    for i in 0..8
        { print!("{} ", seed[i]); }
    println!("-------------------------------");
}

fn random_set_seed_collector()
{
    println!("random_set_seed_collector");
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
    type Func = *const fn() -> [u64; 8];

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    println!("seed = {}", rand.random_u128());
    
    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u64());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    println!("seed = {}", rand.random_u32());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    println!("seed = {}", rand.random_u16());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    println!("seed = {}", rand.random_u8());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u128());
    
    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u64());
    
    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u32());
    
    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u16());
    
    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u8());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u128());
    
    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    println!("seed = {}", slapdash.random_u64());
    
    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    println!("seed = {}", slapdash.random_u32());
    
    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    println!("seed = {}", slapdash.random_u16());
    
    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    println!("seed = {}", slapdash.random_u8());
    
    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    println!("seed = {}", rand.random_u128());
    
    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    println!("seed = {}", any.random_u64());
    
    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    println!("seed = {}", slapdash.random_u32());
    println!("-------------------------------");
}

fn random_reset_seed_collector()
{
    println!("random_reset_seed_collector");
    fn seed_collector() -> [u64; 8]
    {
        [0_u64; 8]
    }
    type Func = *const fn() -> [u64; 8];

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let collector = rand.get_seed_collector();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    rand.reset_seed_collector();
    assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    
    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let collector = rand.get_seed_collector();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    rand.reset_seed_collector();
    assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let collector = rand.get_seed_collector();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    rand.reset_seed_collector();
    assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let collector = rand.get_seed_collector();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    rand.reset_seed_collector();
    assert_eq!(collector as Func, rand.get_seed_collector() as Func);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let collector = slapdash.get_seed_collector();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    slapdash.reset_seed_collector();
    assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    
    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let collector = slapdash.get_seed_collector();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    slapdash.reset_seed_collector();
    assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    
    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let collector = slapdash.get_seed_collector();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    slapdash.reset_seed_collector();
    assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    
    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let collector = slapdash.get_seed_collector();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    slapdash.reset_seed_collector();
    assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    
    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let collector = rand.get_seed_collector();
    rand.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    rand.reset_seed_collector();
    assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    
    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let collector = any.get_seed_collector();
    any.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    any.reset_seed_collector();
    assert_eq!(collector as Func, any.get_seed_collector() as Func);
    
    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let collector = slapdash.get_seed_collector();
    slapdash.set_seed_collector(seed_collector);
    assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    slapdash.reset_seed_collector();
    assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    println!("-------------------------------");
}

fn random_smalluint()
{
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

fn random_biguint()
{
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
    random_random_prime_with_half_length_using_miller_rabin_biguint();
    random_random_prime_with_half_length_using_rsa_biguint();
    random_prepared_random_prime_with_msb_set();
    random_prepared_random_prime_with_half_length();
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

fn random_random_biguint()
{
    println!("random_random_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let biguint: U256 = rand.random_biguint();
    println!("Random Number: {}", biguint);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let biguint: U384 = any.random_biguint();
    println!("Any Number: {}", biguint);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let biguint: U768 = rand.random_biguint();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let biguint: U1024 = rand.random_biguint();
    println!("Random Number: {}", biguint);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let biguint: U2048 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let biguint: U3072 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let biguint: U4096 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let biguint: U5120 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let biguint: U6144 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let biguint: U7168 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let biguint: U8192 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let biguint: U16384 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let biguint: U256 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let biguint: U384 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let biguint: U512 = rand.random_biguint();
    println!("Random Number: {}", biguint);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let biguint: U768 = any.random_biguint();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let biguint: U1024 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let biguint: U2048 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let biguint: U3072 = slapdash.random_biguint();
    println!("Slapdash Number: {}", biguint);
    println!("-------------------------------");
}

fn random_random_under_biguint()
{
    println!("random_random_under_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let ceiling = U16384::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let ceiling = U8192::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let ceiling = U7168::max().wrapping_div_uint(5_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let ceiling = U6144::max().wrapping_div_uint(6_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let ceiling = U5120::max().wrapping_div_uint(7_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let ceiling = U4096::max().wrapping_div_uint(8_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let ceiling = U3072::max().wrapping_div_uint(9_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let ceiling = U2048::max().wrapping_div_uint(10_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let ceiling = U1024::max().wrapping_div_uint(11_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let ceiling = U768::max().wrapping_div_uint(12_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let ceiling = U512::max().wrapping_div_uint(13_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let ceiling = U384::max().wrapping_div_uint(14_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let ceiling = U256::max().wrapping_div_uint(15_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let ceiling = U16384::max().wrapping_div_uint(16_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let ceiling = U8192::max().wrapping_div_uint(17_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let ceiling = U7168::max().wrapping_div_uint(18_u8);
    if let Some(r) = rand.random_under_biguint(&ceiling)
    {
        println!("Random Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let ceiling = U6144::max().wrapping_div_uint(19_u8);
    if let Some(r) = any.random_under_biguint(&ceiling)
    {
        println!("Any Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let ceiling = U5120::max().wrapping_div_uint(20_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let ceiling = U4096::max().wrapping_div_uint(21_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let ceiling = U2048::max().wrapping_div_uint(22_u8);
    if let Some(r) = slapdash.random_under_biguint(&ceiling)
    {
        println!("Slapdash Number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
    }
    println!("-------------------------------");
}

fn random_random_under_biguint_()
{
    println!("random_random_under_biguint_");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let ceiling = U16384::max().wrapping_div_uint(3_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let ceiling = U8192::max().wrapping_div_uint(4_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let ceiling = U7168::max().wrapping_div_uint(5_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let ceiling = U6144::max().wrapping_div_uint(6_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let ceiling = U5120::max().wrapping_div_uint(7_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let ceiling = U4096::max().wrapping_div_uint(8_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let ceiling = U3072::max().wrapping_div_uint(9_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let ceiling = U2048::max().wrapping_div_uint(10_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let ceiling = U1024::max().wrapping_div_uint(11_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let ceiling = U768::max().wrapping_div_uint(12_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let ceiling = U512::max().wrapping_div_uint(13_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let ceiling = U384::max().wrapping_div_uint(14_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let ceiling = U256::max().wrapping_div_uint(15_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let ceiling = U16384::max().wrapping_div_uint(16_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let ceiling = U8192::max().wrapping_div_uint(17_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let ceiling = U7168::max().wrapping_div_uint(18_u8);
    let r = rand.random_under_biguint_(&ceiling);
    println!("Random Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let ceiling = U6144::max().wrapping_div_uint(19_u8);
    let r = any.random_under_biguint_(&ceiling);
    println!("Any Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let ceiling = U5120::max().wrapping_div_uint(20_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let ceiling = U4096::max().wrapping_div_uint(21_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let ceiling = U2048::max().wrapping_div_uint(22_u8);
    let r = slapdash.random_under_biguint_(&ceiling);
    println!("Slapdash Number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    println!("-------------------------------");
}

fn random_random_odd_biguint()
{
    println!("random_random_odd_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let r: U16384 = rand.random_odd_biguint();
    println!("Random odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let r: U8192 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let r: U7168 = rand.random_odd_biguint();
    println!("Random odd number is {}.", r);
    assert!(r.is_odd());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let r: U6144 = rand.random_odd_biguint();
    println!("Random odd number is {}.", r);
    assert!(r.is_odd());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let r: U5120 = rand.random_odd_biguint();
    println!("Random odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let r: U4096 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let r: U3072 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let r: U2048 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let r: U1024 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let r: U768 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let r: U512 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let r: U384 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let r: U256 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let r: U16384 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let r: U8192 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let r: U7168 = rand.random_odd_biguint();
    println!("Random odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let r: U6144 = any.random_odd_biguint();
    println!("Any odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let r: U5120 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let r: U4096 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let r: U3072 = slapdash.random_odd_biguint();
    println!("Slapdash odd number is {}.", r);
    assert!(r.is_odd());
    println!("-------------------------------");
}

fn random_random_odd_under_biguint()
{
    println!("random_random_odd_under_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let ceiling = U16384::max().wrapping_div_uint(3_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let ceiling = U8192::max().wrapping_div_uint(4_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let ceiling = U7168::max().wrapping_div_uint(5_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let ceiling = U6144::max().wrapping_div_uint(6_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let ceiling = U5120::max().wrapping_div_uint(7_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let ceiling = U4096::max().wrapping_div_uint(8_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let ceiling = U3072::max().wrapping_div_uint(9_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let ceiling = U2048::max().wrapping_div_uint(10_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let ceiling = U1024::max().wrapping_div_uint(11_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let ceiling = U768::max().wrapping_div_uint(12_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let ceiling = U512::max().wrapping_div_uint(13_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let ceiling = U384::max().wrapping_div_uint(14_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let ceiling = U256::max().wrapping_div_uint(15_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let ceiling = U16384::max().wrapping_div_uint(16_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let ceiling = U8192::max().wrapping_div_uint(17_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let ceiling = U7168::max().wrapping_div_uint(18_u8);
    if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    {
        println!("Random odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let ceiling = U6144::max().wrapping_div_uint(19_u8);
    if let Some(r) = any.random_odd_under_biguint(&ceiling)
    {
        println!("Any odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let ceiling = U5120::max().wrapping_div_uint(20_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let ceiling = U4096::max().wrapping_div_uint(21_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let ceiling = U2048::max().wrapping_div_uint(22_u8);
    if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    {
        println!("Slapdash odd number less than {} is\n{}", ceiling, r);
        assert!(r < ceiling);
        assert!(r.is_odd());
    }
    println!("-------------------------------");
}

fn random_random_odd_under_biguint_()
{
    println!("random_random_odd_under_biguint_");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let ceiling = U16384::max().wrapping_div_uint(3_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let ceiling = U8192::max().wrapping_div_uint(4_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let ceiling = U7168::max().wrapping_div_uint(5_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let ceiling = U6144::max().wrapping_div_uint(6_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let ceiling = U5120::max().wrapping_div_uint(7_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let ceiling = U4096::max().wrapping_div_uint(8_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let ceiling = U3072::max().wrapping_div_uint(9_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let ceiling = U2048::max().wrapping_div_uint(10_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let ceiling = U1024::max().wrapping_div_uint(11_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let ceiling = U768::max().wrapping_div_uint(12_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let ceiling = U512::max().wrapping_div_uint(13_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let ceiling = U384::max().wrapping_div_uint(14_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let ceiling = U256::max().wrapping_div_uint(15_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let ceiling = U16384::max().wrapping_div_uint(16_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let ceiling = U8192::max().wrapping_div_uint(17_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let ceiling = U7168::max().wrapping_div_uint(18_u8);
    let r = rand.random_odd_under_biguint_(&ceiling);
    println!("Random odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let ceiling = U6144::max().wrapping_div_uint(19_u8);
    let r = any.random_odd_under_biguint_(&ceiling);
    println!("Any odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let ceiling = U5120::max().wrapping_div_uint(20_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let ceiling = U4096::max().wrapping_div_uint(21_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let ceiling = U2048::max().wrapping_div_uint(22_u8);
    let r = slapdash.random_odd_under_biguint_(&ceiling);
    println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    assert!(r < ceiling);
    assert!(r.is_odd());
    println!("-------------------------------");
}

fn random_random_with_msb_set_biguint()
{
    println!("random_random_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let r: U16384 = rand.random_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U16384::halfmax());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let r: U8192 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U8192::halfmax());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let r: U7168 = rand.random_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U7168::halfmax());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let r: U6144 = rand.random_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U6144::halfmax());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let r: U5120 = rand.random_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U5120::halfmax());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let r: U4096 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U4096::halfmax());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let r: U3072 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U3072::halfmax());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let r: U2048 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U2048::halfmax());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let r: U1024 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U1024::halfmax());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let r: U768 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U768::halfmax());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let r: U512 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U512::halfmax());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let r: U384 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U384::halfmax());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let r: U256 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U256::halfmax());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let r: U16384 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U16384::halfmax());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let r: U8192 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U8192::halfmax());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let r: U7168 = rand.random_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U7168::halfmax());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let r: U6144 = any.random_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U6144::halfmax());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let r: U5120 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U5120::halfmax());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let r: U4096 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U4096::halfmax());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let r: U3072 = slapdash.random_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U3072::halfmax());
    println!("-------------------------------");
}

fn random_random_odd_with_msb_set_biguint()
{
    println!("random_random_odd_with_msb_set_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let r: U16384 = rand.random_odd_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U16384::halfmax());
    assert!(r.is_odd());

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let r: U8192 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U8192::halfmax());
    assert!(r.is_odd());
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let r: U7168 = rand.random_odd_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U7168::halfmax());
    assert!(r.is_odd());
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let r: U6144 = rand.random_odd_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U6144::halfmax());
    assert!(r.is_odd());
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let r: U5120 = rand.random_odd_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U5120::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let r: U4096 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U4096::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let r: U3072 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U3072::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let r: U2048 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U2048::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let r: U1024 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U1024::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let r: U768 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U768::halfmax());
    assert!(r.is_odd());

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let r: U512 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U512::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let r: U384 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U384::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let r: U256 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U256::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let r: U16384 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U16384::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let r: U8192 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U8192::halfmax());
    assert!(r.is_odd());

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let r: U7168 = rand.random_odd_with_msb_set_biguint();
    println!("Random number is {}.", r);
    assert!(r > U7168::halfmax());
    assert!(r.is_odd());

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let r: U6144 = any.random_odd_with_msb_set_biguint();
    println!("Any number is {}.", r);
    assert!(r > U6144::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let r: U5120 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U5120::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let r: U4096 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U4096::halfmax());
    assert!(r.is_odd());

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let r: U3072 = slapdash.random_odd_with_msb_set_biguint();
    println!("Slapdash number is {}.", r);
    assert!(r > U3072::halfmax());
    assert!(r.is_odd());
    println!("-------------------------------");
}

fn random_random_prime_using_miller_rabin_biguint()
{
    println!("random_random_prime_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut thread = Vec::<fn()>::new();

    // Example for Random
    thread.push(||{
        use cryptocol::random::Random;
        let mut rand = Random::new();
        let prime: U256 = rand.random_prime_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any
    thread.push(||{
        use cryptocol::random::Any;
        let mut any = Any::new();
        let prime: U384 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });
    
    // Example for Random_BIG_KECCAK_1024
    thread.push(||{
        use cryptocol::random::Random_BIG_KECCAK_1024;
        let mut rand = Random_BIG_KECCAK_1024::new();
        let prime: U512 = rand.random_prime_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA3_512
    thread.push(||{
        use cryptocol::random::Random_SHA3_512;
        let mut rand = Random_SHA3_512::new();
        let prime: U768 = rand.random_prime_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA2_512
    thread.push(||{
        use cryptocol::random::Random_SHA2_512;
        let mut rand = Random_SHA2_512::new();
        let prime: U1024 = rand.random_prime_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_SHAKE_256
    thread.push(||{
        use cryptocol::random::Any_SHAKE_256;
        let mut any = Any_SHAKE_256::new();
        let prime: U2048 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHAKE_128
    thread.push(||{
        use cryptocol::random::Any_SHAKE_128;
        let mut any = Any_SHAKE_128::new();
        let prime: U3072 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_512
    thread.push(||{
        use cryptocol::random::Any_SHA3_512;
        let mut any = Any_SHA3_512::new();
        let prime: U4096 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_256
    thread.push(||{
        use cryptocol::random::Any_SHA3_256;
        let mut any = Any_SHA3_256::new();
        let prime: U5120 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_512
    thread.push(||{
        use cryptocol::random::Any_SHA2_512;
        let mut any = Any_SHA2_512::new();
        let prime: U6144 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_256
    thread.push(||{
        use cryptocol::random::Any_SHA2_256;
        let mut any = Any_SHA2_256::new();
        let prime: U7168 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_SHA1
    thread.push(||{
        use cryptocol::random::Slapdash_SHA1;
        let mut slapdash = Slapdash_SHA1::new();
        let prime: U8192 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_SHA0
    thread.push(||{
        use cryptocol::random::Slapdash_SHA0;
        let mut slapdash = Slapdash_SHA0::new();
        let prime: U16384 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD5
    thread.push(||{
        use cryptocol::random::Slapdash_MD5;
        let mut slapdash = Slapdash_MD5::new();
        let prime: U256 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD4
    thread.push(||{
        use cryptocol::random::Slapdash_MD4;
        let mut slapdash = Slapdash_MD4::new();
        let prime: U384 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Random_Rijndael
    thread.push(||{
        use cryptocol::random::Random_Rijndael;
        let mut rand = Random_Rijndael::new();
        let prime: U512 = rand.random_prime_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_Rijndael
    thread.push(||{
        use cryptocol::random::Any_Rijndael;
        let mut any = Any_Rijndael::new();
        let prime: U768 = any.random_prime_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_DES
    thread.push(||{
        use cryptocol::random::Slapdash_DES;
        let mut slapdash = Slapdash_DES::new();
        let prime: U1024 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_Num_C
    thread.push(||{
        use cryptocol::random::Slapdash_Num_C;
        let mut slapdash = Slapdash_Num_C::new();
        let prime: U2048 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash
    thread.push(||{
        use cryptocol::random::Slapdash;
        let mut slapdash = Slapdash::new();
        let prime: U3072 = slapdash.random_prime_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_random_prime_with_msb_set_using_miller_rabin_biguint()
{
    println!("random_random_prime_with_msb_set_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut thread = Vec::<fn()>::new();

    // Example for Random
    thread.push(||{
        use cryptocol::random::Random;
        let mut rand = Random::new();
        let prime: U256 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any
    thread.push(||{
        use cryptocol::random::Any;
        let mut any = Any::new();
        let prime: U384 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });
    
    // Example for Random_BIG_KECCAK_1024
    thread.push(||{
        use cryptocol::random::Random_BIG_KECCAK_1024;
        let mut rand = Random_BIG_KECCAK_1024::new();
        let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA3_512
    thread.push(||{
        use cryptocol::random::Random_SHA3_512;
        let mut rand = Random_SHA3_512::new();
        let prime: U768 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA2_512
    thread.push(||{
        use cryptocol::random::Random_SHA2_512;
        let mut rand = Random_SHA2_512::new();
        let prime: U1024 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_SHAKE_256
    thread.push(||{
        use cryptocol::random::Any_SHAKE_256;
        let mut any = Any_SHAKE_256::new();
        let prime: U2048 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHAKE_128
    thread.push(||{
        use cryptocol::random::Any_SHAKE_128;
        let mut any = Any_SHAKE_128::new();
        let prime: U3072 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_512
    thread.push(||{
        use cryptocol::random::Any_SHA3_512;
        let mut any = Any_SHA3_512::new();
        let prime: U4096 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_256
    thread.push(||{
        use cryptocol::random::Any_SHA3_256;
        let mut any = Any_SHA3_256::new();
        let prime: U5120 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_512
    thread.push(||{
        use cryptocol::random::Any_SHA2_512;
        let mut any = Any_SHA2_512::new();
        let prime: U6144 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_256
    thread.push(||{
        use cryptocol::random::Any_SHA2_256;
        let mut any = Any_SHA2_256::new();
        let prime: U7168 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_SHA1
    thread.push(||{
        use cryptocol::random::Slapdash_SHA1;
        let mut slapdash = Slapdash_SHA1::new();
        let prime: U8192 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_SHA0
    thread.push(||{
        use cryptocol::random::Slapdash_SHA0;
        let mut slapdash = Slapdash_SHA0::new();
        let prime: U16384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD5
    thread.push(||{
        use cryptocol::random::Slapdash_MD5;
        let mut slapdash = Slapdash_MD5::new();
        let prime: U256 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD4
    thread.push(||{
        use cryptocol::random::Slapdash_MD4;
        let mut slapdash = Slapdash_MD4::new();
        let prime: U384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Random_Rijndael
    thread.push(||{
        use cryptocol::random::Random_Rijndael;
        let mut rand = Random_Rijndael::new();
        let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_Rijndael
    thread.push(||{
        use cryptocol::random::Any_Rijndael;
        let mut any = Any_Rijndael::new();
        let prime: U768 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_DES
    thread.push(||{
        use cryptocol::random::Slapdash_DES;
        let mut slapdash = Slapdash_DES::new();
        let prime: U1024 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_Num_C
    thread.push(||{
        use cryptocol::random::Slapdash_Num_C;
        let mut slapdash = Slapdash_Num_C::new();
        let prime: U2048 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash
    thread.push(||{
        use cryptocol::random::Slapdash;
        let mut slapdash = Slapdash::new();
        let prime: U3072 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_random_prime_with_half_length_using_miller_rabin_biguint()
{
    println!("random_random_prime_with_half_length_using_miller_rabin_biguint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut thread = Vec::<fn()>::new();

    // Example for Random
    thread.push(||{
        use cryptocol::random::Random;
        let mut rand = Random::new();
        let prime: U256 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any
    thread.push(||{
        use cryptocol::random::Any;
        let mut any = Any::new();
        let prime: U384 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });
    
    // Example for Random_BIG_KECCAK_1024
    thread.push(||{
        use cryptocol::random::Random_BIG_KECCAK_1024;
        let mut rand = Random_BIG_KECCAK_1024::new();
        let prime: U512 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA3_512
    thread.push(||{
        use cryptocol::random::Random_SHA3_512;
        let mut rand = Random_SHA3_512::new();
        let prime: U768 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });
    
    // Example for Random_SHA2_512
    thread.push(||{
        use cryptocol::random::Random_SHA2_512;
        let mut rand = Random_SHA2_512::new();
        let prime: U1024 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_SHAKE_256
    thread.push(||{
        use cryptocol::random::Any_SHAKE_256;
        let mut any = Any_SHAKE_256::new();
        let prime: U2048 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHAKE_128
    thread.push(||{
        use cryptocol::random::Any_SHAKE_128;
        let mut any = Any_SHAKE_128::new();
        let prime: U3072 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_512
    thread.push(||{
        use cryptocol::random::Any_SHA3_512;
        let mut any = Any_SHA3_512::new();
        let prime: U4096 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA3_256
    thread.push(||{
        use cryptocol::random::Any_SHA3_256;
        let mut any = Any_SHA3_256::new();
        let prime: U5120 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_512
    thread.push(||{
        use cryptocol::random::Any_SHA2_512;
        let mut any = Any_SHA2_512::new();
        let prime: U6144 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Any_SHA2_256
    thread.push(||{
        use cryptocol::random::Any_SHA2_256;
        let mut any = Any_SHA2_256::new();
        let prime: U7168 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_SHA1
    thread.push(||{
        use cryptocol::random::Slapdash_SHA1;
        let mut slapdash = Slapdash_SHA1::new();
        let prime: U8192 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_SHA0
    thread.push(||{
        use cryptocol::random::Slapdash_SHA0;
        let mut slapdash = Slapdash_SHA0::new();
        let prime: U16384 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD5
    thread.push(||{
        use cryptocol::random::Slapdash_MD5;
        let mut slapdash = Slapdash_MD5::new();
        let prime: U256 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_MD4
    thread.push(||{
        use cryptocol::random::Slapdash_MD4;
        let mut slapdash = Slapdash_MD4::new();
        let prime: U384 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Random_Rijndael
    thread.push(||{
        use cryptocol::random::Random_Rijndael;
        let mut rand = Random_Rijndael::new();
        let prime: U512 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Random prime number: {}", prime);
    });

    // Example for Any_Rijndael
    thread.push(||{
        use cryptocol::random::Any_Rijndael;
        let mut any = Any_Rijndael::new();
        let prime: U768 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Any prime number: {}", prime);
    });

    // Example for Slapdash_DES
    thread.push(||{
        use cryptocol::random::Slapdash_DES;
        let mut slapdash = Slapdash_DES::new();
        let prime: U1024 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash_Num_C
    thread.push(||{
        use cryptocol::random::Slapdash_Num_C;
        let mut slapdash = Slapdash_Num_C::new();
        let prime: U2048 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    // Example for Slapdash
    thread.push(||{
        use cryptocol::random::Slapdash;
        let mut slapdash = Slapdash::new();
        let prime: U3072 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
        println!("Slapdash prime number: {}", prime);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn random_random_prime_with_half_length_using_rsa_biguint()
{
    println!("random_random_prime_with_half_length_using_rsa_biguint");
    use cryptocol::random::Random;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut prng = Random::new();
    let (prime1, prime2): (U1024, U1024) = prng.random_prime_with_half_length_using_rsa_biguint(7);
    let (prime1, prime2): (U512, U512) = (prime1.into_biguint(), prime2.into_biguint());
    println!("U512 Prime number: {}", prime1);
    println!("U512 Prime number: {}", prime2);
    println!("-------------------------------");
}

fn random_prepared_random_prime_with_msb_set()
{
    println!("random_prepared_random_prime_with_msb_set");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let biguint: U256 = rand.prepared_random_prime_with_msb_set();
    println!("Random Number: {}", biguint);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let biguint: U384 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let biguint: U512 = rand.prepared_random_prime_with_msb_set();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let biguint: U768 = rand.prepared_random_prime_with_msb_set();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let biguint: U1024 = rand.prepared_random_prime_with_msb_set();
    println!("Random Number: {}", biguint);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let biguint: U2048 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let biguint: U3072 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let biguint: U4096 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let biguint: U5120 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let biguint: U6144 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let biguint: U7168 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let biguint: U8192 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let biguint: U16384 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let biguint: U256 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let biguint: U384 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let biguint: U512 = rand.prepared_random_prime_with_msb_set();
    println!("Random Number: {}", biguint);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let biguint: U768 = any.prepared_random_prime_with_msb_set();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let biguint: U1024 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let biguint: U2048 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let biguint: U3072 = slapdash.prepared_random_prime_with_msb_set();
    println!("Slapdash Number: {}", biguint);
    println!("-------------------------------");
}

fn random_prepared_random_prime_with_half_length()
{
    println!("random_prepared_random_prime_with_half_length");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Example for Random
    use cryptocol::random::Random;
    let mut rand = Random::new();
    let biguint: U256 = rand.prepared_random_prime_with_half_length();
    println!("Random Number: {}", biguint);

    // Example for Any
    use cryptocol::random::Any;
    let mut any = Any::new();
    let biguint: U384 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);
    
    // Example for Random_BIG_KECCAK_1024
    use cryptocol::random::Random_BIG_KECCAK_1024;
    let mut rand = Random_BIG_KECCAK_1024::new();
    let biguint: U512 = rand.prepared_random_prime_with_half_length();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA3_512
    use cryptocol::random::Random_SHA3_512;
    let mut rand = Random_SHA3_512::new();
    let biguint: U768 = rand.prepared_random_prime_with_half_length();
    println!("Random Number: {}", biguint);
    
    // Example for Random_SHA2_512
    use cryptocol::random::Random_SHA2_512;
    let mut rand = Random_SHA2_512::new();
    let biguint: U1024 = rand.prepared_random_prime_with_half_length();
    println!("Random Number: {}", biguint);

    // Example for Any_SHAKE_256
    use cryptocol::random::Any_SHAKE_256;
    let mut any = Any_SHAKE_256::new();
    let biguint: U2048 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Any_SHAKE_128
    use cryptocol::random::Any_SHAKE_128;
    let mut any = Any_SHAKE_128::new();
    let biguint: U3072 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_512
    use cryptocol::random::Any_SHA3_512;
    let mut any = Any_SHA3_512::new();
    let biguint: U4096 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA3_256
    use cryptocol::random::Any_SHA3_256;
    let mut any = Any_SHA3_256::new();
    let biguint: U5120 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_512
    use cryptocol::random::Any_SHA2_512;
    let mut any = Any_SHA2_512::new();
    let biguint: U6144 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Any_SHA2_256
    use cryptocol::random::Any_SHA2_256;
    let mut any = Any_SHA2_256::new();
    let biguint: U7168 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_SHA1
    use cryptocol::random::Slapdash_SHA1;
    let mut slapdash = Slapdash_SHA1::new();
    let biguint: U8192 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_SHA0
    use cryptocol::random::Slapdash_SHA0;
    let mut slapdash = Slapdash_SHA0::new();
    let biguint: U16384 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD5
    use cryptocol::random::Slapdash_MD5;
    let mut slapdash = Slapdash_MD5::new();
    let biguint: U256 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_MD4
    use cryptocol::random::Slapdash_MD4;
    let mut slapdash = Slapdash_MD4::new();
    let biguint: U384 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Random_Rijndael
    use cryptocol::random::Random_Rijndael;
    let mut rand = Random_Rijndael::new();
    let biguint: U512 = rand.prepared_random_prime_with_half_length();
    println!("Random Number: {}", biguint);

    // Example for Any_Rijndael
    use cryptocol::random::Any_Rijndael;
    let mut any = Any_Rijndael::new();
    let biguint: U768 = any.prepared_random_prime_with_half_length();
    println!("Any Number: {}", biguint);

    // Example for Slapdash_DES
    use cryptocol::random::Slapdash_DES;
    let mut slapdash = Slapdash_DES::new();
    let biguint: U1024 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash_Num_C
    use cryptocol::random::Slapdash_Num_C;
    let mut slapdash = Slapdash_Num_C::new();
    let biguint: U2048 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);

    // Example for Slapdash
    use cryptocol::random::Slapdash;
    let mut slapdash = Slapdash::new();
    let biguint: U3072 = slapdash.prepared_random_prime_with_half_length();
    println!("Slapdash Number: {}", biguint);
    println!("-------------------------------");
}

const COLUMN: usize = 100;
fn random_prepare_primes()
{
    println!("random_prepare_primes");
    use cryptocol::asymmetric::{ RSA_Generic, RSA_1024, RSA_2048, RSA_4096 };
    use cryptocol::random::RandGen;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    #[allow(non_snake_case)]
    let NUM_STR = RandGen::prepared_random_prime_numbers();

    // find_u2048_primes();
    // return;

    #[allow(non_camel_case_types)] type RSA_512 = RSA_Generic<16, u32>;
    let mut thread = Vec::<(&'static [&'static str; COLUMN], usize, fn(num_str: &'static [&'static str; COLUMN], i: usize))>::new();

    for i in 1..COLUMN
    {
        thread.push((&NUM_STR[3], i, |num_str: &'static [&'static str; COLUMN], i: usize|{
            let prime1 = U512::from_str_radix(num_str[0], 16).unwrap();
            let prime2 = U512::from_str_radix(num_str[i], 16).unwrap();
            let rsa = RSA_512::new_with_primes(prime1, prime2);
            let message = rsa.get_modulus().shift_right(512_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover != message
                { println!("U256 prime: {i}th Wrong Prime."); }
        }));
    }

    for i in 1..COLUMN
    {
        thread.push((&NUM_STR[2], i, |num_str: &'static [&'static str; COLUMN], i: usize|{
            let prime1 = U1024::from_str_radix(num_str[0], 16).unwrap();
            let prime2 = U1024::from_str_radix(num_str[i], 16).unwrap();
            let rsa = RSA_1024::new_with_primes(prime1, prime2);
            let message = rsa.get_modulus().shift_right(1024_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover != message
                { println!("U512 prime: {i}th Wrong Prime."); }
        }));
    }

    for i in 1..COLUMN
    {
        thread.push((&NUM_STR[1], i, |num_str: &'static [&'static str; COLUMN], i: usize|{
            let prime1 = U2048::from_str_radix(num_str[0], 16).unwrap();
            let prime2 = U2048::from_str_radix(num_str[i], 16).unwrap();
            let rsa = RSA_2048::new_with_primes(prime1, prime2);
            let message = rsa.get_modulus().shift_right(2048_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover != message
                { println!("U1024 prime: {i}th Wrong Prime."); }
        }));
    }

    for i in 1..COLUMN
    {
        thread.push((&NUM_STR[0], i, |num_str: &'static [&'static str; COLUMN], i: usize|{
            let prime1 = U4096::from_str_radix(num_str[0], 16).unwrap();
            let prime2 = U4096::from_str_radix(num_str[i], 16).unwrap();
            let rsa = RSA_4096::new_with_primes(prime1, prime2);
            let message = rsa.get_modulus().shift_right(4096_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover != message
                { println!("U2048 prime: {i}th Wrong Prime."); }
        }));
    }

    do_with_arg_simultaneously(thread);
    println!("-------------------------------");
}

fn do_with_arg_simultaneously(jobs: Vec<(&'static [&'static str; COLUMN], usize, fn(&'static [&'static str; COLUMN], usize))>)
{
    let number_of_threads: usize = match available_parallelism()
    {
        Ok(non_zero) => non_zero.get() as usize,
        Err(_) => 1_usize,
    };
    
    if number_of_threads == 1
    {
        for (num_str, i, work) in jobs
            { work(num_str, i); }
        return;
    }

    let mut threads = Vec::new();
    let (tx, rx) = channel::<(&'static [&'static str; COLUMN], usize, fn(&'static [&'static str; COLUMN], usize))>();
    let receiver = Arc::new(Mutex::new(rx));
    for _ in 0..number_of_threads
    {
        let rxx = receiver.clone();
        threads.push(spawn(move ||
        {
            loop
            {
                let r = rxx.lock().unwrap();
                match r.recv()
                {
                    Ok((num_str, i, work)) => { work(num_str, i); drop(r); },
                    _ => { return },
                }
            }
        }));
    }

    for job in jobs
        { tx.clone().send(job).unwrap(); }
    drop(tx);
    for thread in threads
        { thread.join().unwrap(); }
}

fn find_u256_primes()
{
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;
    use cryptocol::asymmetric::RSA_Generic;
    define_utypes_with!(u32);

    type PRIME = U512;
    type RSA = RSA_Generic<16, u32>;
    const NUM_STR: &str = "B438E6DC10A4FD3C54C2D8CFA523352D7E0EBBC0698DDE11634CD9C74A4BC4B3";

    let mut thread = Vec::<fn()>::new();
    for _ in 0..50
    {
        thread.push(||{
            let mut prng = Random::new();
            let (prime1, prime2): (PRIME, PRIME) = prng.random_prime_with_half_length_using_rsa_biguint(7);
            let base = PRIME::from_str_radix(NUM_STR, 16).unwrap();

            let rsa = RSA::new_with_primes(prime1.clone(), base.clone());
            let message = rsa.get_modulus().shift_right(512_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U256 prime: prime1 is {:X}", prime1); }
            else
                { println!("U156 prime: prime1 is Wrong Prime."); }

            let rsa = RSA::new_with_primes(prime2.clone(), base);
            let message = rsa.get_modulus().shift_right(512_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U256 prime: prime2 is {:X}", prime2); }
            else
                { println!("U256 prime: prime2 is Wrong Prime."); }
        });
    }

    do_simultaneously(thread);
}

fn find_u512_primes()
{
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;
    use cryptocol::asymmetric::RSA_1024;
    define_utypes_with!(u32);

    type PRIME = U1024;
    type RSA = RSA_1024;
    const NUM_STR: &str = "950BE5031347033FAD37E4AA8FBA7B9687432E00C8D5E7829B0366B5E602FB308513C315D751E9F704127BFAD3995001765A47BD45C213E3CE22E5142C279F39";

    let mut thread = Vec::<fn()>::new();
    for _ in 0..50
    {
        thread.push(||{
            let mut prng = Random::new();
            let (prime1, prime2): (PRIME, PRIME) = prng.random_prime_with_half_length_using_rsa_biguint(7);
            let base = PRIME::from_str_radix(NUM_STR, 16).unwrap();

            let rsa = RSA::new_with_primes(prime1.clone(), base.clone());
            let message = rsa.get_modulus().shift_right(1024_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U512 prime: prime1 is {:X}", prime1); }
            else
                { println!("U512 prime: prime1 is Wrong Prime."); }

            let rsa = RSA::new_with_primes(prime2.clone(), base);
            let message = rsa.get_modulus().shift_right(1024_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U512 prime: prime2 is {:X}", prime2); }
            else
                { println!("U512 prime: prime2 is Wrong Prime."); }
        });
    }

    do_simultaneously(thread);
}

fn find_u1024_primes()
{
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;
    use cryptocol::asymmetric::RSA_2048;
    define_utypes_with!(u32);

    type PRIME = U2048;
    type RSA = RSA_2048;
    const NUM_STR: &str = "EDA36FC2173A4905961C3772EE419804D2CE8E30AF11DFFED65A99D73571BE321005E9B9DD520FD889EEFF8C6887436A9D37E9033A8FCADC360564E3D5DDF8D12EC55A328968B4C4EFBE7F7410276D448ACD6692836ADCBEC6BAD8B58935916DE3D67FDE6BB4D1C1047FAC556D33E8CA2EEED508C014E5B9B9A76AC06FA27AC1";

    let mut thread = Vec::<fn()>::new();
    for _ in 0..50
    {
        thread.push(||{
            let mut prng = Random::new();
            let (prime1, prime2): (PRIME, PRIME) = prng.random_prime_with_half_length_using_rsa_biguint(7);
            let base = PRIME::from_str_radix(NUM_STR, 16).unwrap();
 
            let rsa = RSA::new_with_primes(prime1.clone(), base.clone());
            let message = rsa.get_modulus().shift_right(2048_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U1024 prime: prime1 is {:X}", prime1); }
            else
                { println!("U1024 prime: prime1 is Wrong Prime."); }

            let rsa = RSA::new_with_primes(prime2.clone(), base);
            let message = rsa.get_modulus().shift_right(2048_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U1024 prime: prime2 is {:X}", prime2); }
            else
                { println!("U1024 prime: prime2 is Wrong Prime."); }
        });
    }

    do_simultaneously(thread);
}

fn find_u2048_primes()
{
    use cryptocol::define_utypes_with;
    use cryptocol::random::Random;
    use cryptocol::asymmetric::RSA_4096;
    define_utypes_with!(u32);

    type PRIME = U4096;
    type RSA = RSA_4096;
    const NUM_STR: &str = "DF5EE9D60161F39444CE204C5825BC56B3110B774A66B5CCB188E405C38AF82372C4B497B2A8044E6BCCD06226F7C406CF0CA06950A2544693C6286D8CA7855B3479E3C9E99F4B7F434E7DD2BF457C81C07D183F63C546237393B0ABD5055026B7F5AE974E9965FC5B7F3F7F0DA1DB29387232178DF14E6A4D1521DF8241B1C2EEA56A060686DF0C60C326F956EC6C0DFAD1F7DA926253EAC05D992412BF6B69C123385092F95EF9A67FF2E45C744FA28197438E0BF7E9979EF52E277C60A615CEF051B35DBE01EC6A71E47798FDED006226FC8F78C3A88D20A00406D1C062D10D94D475A639B32A6DA53348FC84E9B5E150E01D4F7EB1D8B035575CE56799C5";

    let mut thread = Vec::<fn()>::new();
    for _ in 0..50
    {
        thread.push(||{
            let mut prng = Random::new();
            let (prime1, prime2): (PRIME, PRIME) = prng.random_prime_with_half_length_using_rsa_biguint(7);
            let base = PRIME::from_str_radix(NUM_STR, 16).unwrap();

            let rsa = RSA::new_with_primes(prime1.clone(), base.clone());
            let message = rsa.get_modulus().shift_right(4096_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U2048 prime: prime1 is {:X}", prime1); }
            else
                { println!("U2048 prime: prime1 is Wrong Prime."); }

            let rsa = RSA::new_with_primes(prime2.clone(), base);
            let message = rsa.get_modulus().shift_right(2048_usize >> 1);
            let cipher = rsa.encrypt_biguint(&message);
            let recover = rsa.decrypt_biguint(&cipher);
            if recover == message
                { println!("U2048 prime: prime2 is {:X}", prime2); }
            else
                { println!("U2048 prime: prime2 is Wrong Prime."); }
        });
    }

    do_simultaneously(thread);
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

fn finding()
{
    use cryptocol::random::Random;
    for _ in 0..10
    {
        let mut prng = Random::new();
        let (prime1, prime2) = prng.random_prime_with_half_length_using_rsa_biguint();
        let base = U512::from_str_radix(NUM_STR[3][0], 16).unwrap();

        let rsa = RSA_512::new_with_primes(prime1.clone(), base.clone());
        let message = rsa.get_modulus().wrapping_div_uint(3_u32).wrapping_mul_uint(2_u32);
        let cipher = rsa.encrypt_biguint(&message);
        let recover = rsa.decrypt_biguint(&cipher);
        if recover == message
            { println!("U256 prime: prime1 is {:X}", prime1); }
        else
            { println!("U256 prime: prime1 is Wrong Prime."); }

        let rsa = RSA_512::new_with_primes(prime2.clone(), base);
        let message = rsa.get_modulus().wrapping_div_uint(3_u32).wrapping_mul_uint(2_u32);
        let cipher = rsa.encrypt_biguint(&message);
        let recover = rsa.decrypt_biguint(&cipher);
        if recover == message
            { println!("U256 prime: prime2 is {:X}", prime2); }
        else
            { println!("U256 prime: prime2 is Wrong Prime."); }
    }
}
*/
