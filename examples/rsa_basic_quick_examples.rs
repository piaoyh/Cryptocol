// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]


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
    rsa_quick_start();
    rsa_new();
    rsa_new_with_automatic_keys();
    rsa_new_with_keys();
    rsa_new_with_primes();
    rsa_new_with_prepared_keys();
    // rsa_same_primes_test();
    // rsa_composites_test();
    // rsa_composite_prime_test();
}

fn rsa_quick_start()
{
    rsa_import();
}

#[allow(unused_imports)]
fn rsa_import()
{
    println!("rsa_import");
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::asymmetric::RSA_2048;
    use cryptocol::asymmetric::RSA_4096;
    use cryptocol::asymmetric::RSA_Generic;
    use cryptocol::asymmetric::RSA_1024_u128;
    use cryptocol::asymmetric::RSA_2048_u128;
    use cryptocol::asymmetric::RSA_4096_u128;
    use cryptocol::asymmetric::RSA_1024_u64;
    use cryptocol::asymmetric::RSA_2048_u64;
    use cryptocol::asymmetric::RSA_4096_u64;
    use cryptocol::asymmetric::RSA_1024_u32;
    use cryptocol::asymmetric::RSA_2048_u32;
    use cryptocol::asymmetric::RSA_4096_u32;
    use cryptocol::asymmetric::RSA_1024_u16;
    use cryptocol::asymmetric::RSA_2048_u16;
    use cryptocol::asymmetric::RSA_4096_u16;
    use cryptocol::asymmetric::RSA_1024_u8;
    use cryptocol::asymmetric::RSA_2048_u8;
    use cryptocol::asymmetric::RSA_4096_u8;
    println!("-------------------------------");
}

fn rsa_new()
{
    println!("rsa_new");
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    let _rsa = RSA_1024::new();
    
    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;
    let _rsa = RSA_2048::new();
    
    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;
    let _rsa = RSA_4096::new();
    
    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;
    let _rsa = RSA_Generic::<8, u32, 5>::new();
    
    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;
    let _rsa = RSA_1024_u128::new();
    
    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;
    let _rsa = RSA_2048_u128::new();
    
    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;
    let _rsa = RSA_4096_u128::new();
    
    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;
    let _rsa = RSA_1024_u64::new();
    
    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;
    let _rsa = RSA_2048_u64::new();
    
    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;
    let _rsa = RSA_4096_u64::new();
    
    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;
    let _rsa = RSA_1024_u32::new();
    
    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;
    let _rsa = RSA_2048_u32::new();
    
    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;
    let _rsa = RSA_4096_u32::new();
    
    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;
    let _rsa = RSA_1024_u16::new();
    
    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;
    let _rsa = RSA_2048_u16::new();
    
    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;
    let _rsa = RSA_4096_u16::new();
    
    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;
    let _rsa = RSA_1024_u8::new();
    
    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;
    let _rsa = RSA_2048_u8::new();
    
    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;
    let _rsa = RSA_4096_u8::new();
    println!("-------------------------------");
}

fn rsa_new_with_automatic_keys()
{
    println!("rsa_new_with_automatic_keys");

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024;

        let rsa = RSA_1024::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let rsa = RSA_2048::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let rsa = RSA_4096::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let rsa = RSA_Generic::<8, u32, 5>::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let rsa = RSA_1024_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let rsa = RSA_2048_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let rsa = RSA_4096_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let rsa = RSA_1024_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let rsa = RSA_2048_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let rsa = RSA_4096_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let rsa = RSA_1024_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let rsa = RSA_2048_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let rsa = RSA_4096_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let rsa = RSA_1024_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let rsa = RSA_2048_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let rsa = RSA_4096_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let rsa = RSA_1024_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let rsa = RSA_2048_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let rsa = RSA_4096_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulo);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn rsa_new_with_keys()
{
    println!("rsa_new_with_keys");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulo = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024::new_with_keys(public_key, private_key, modulo);
    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulo = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048::new_with_keys(public_key, private_key, modulo);
    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulo = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096::new_with_keys(public_key, private_key, modulo);
    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let private_key = U512::from_str_radix("46CAC62F340DCC24A6FD1C603B57B4EE361C7C4370B01D769E53BC74CABC6153", 16).unwrap();
    let public_key = U512::from(3_u8);
    let modulo = U512::from_str_radix("6a302946ce14b236fa7baa9059038f669cf8c4f1988fe647d4583bd600fecf6d", 16).unwrap();

    let rsa = RSA_Generic::<16, u32, 5>::new_with_keys(public_key, private_key, modulo);
    println!("RSA_Generic: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    println!("RSA_Generic: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    println!("-------------------------------");
}

fn rsa_new_with_primes()
{
    println!("rsa_new_with_primes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u16::new_with_primes(prime1, prime2);
        println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
        println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u16::new_with_primes(prime1, prime2);
        println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
        println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u16::new_with_primes(prime1, prime2);
        println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
        println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let prime1 = U256::from_str_radix("83CD305566CAC6013CC9A4230B040C64CFA7E50948208099BFB2744DD0DE8307", 16).unwrap();
        let prime2 = U256::from_str_radix("EA2A0C0B9696A6249323A66E5B4C2A8DD2E3417D3F9B3F48E603AD85943547DF", 16).unwrap();

        let rsa = RSA_Generic::<32, u16, 5>::new_with_primes(prime1, prime2);
        println!("RSA_Generic: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
        println!("RSA_Generic: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn rsa_new_with_prepared_keys()
{
    println!("rsa_new_with_prepared_keys");

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024;

        let rsa = RSA_1024::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let rsa = RSA_2048::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let rsa = RSA_4096::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let rsa = RSA_Generic::<8, u32, 5>::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let rsa = RSA_1024_u128::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let rsa = RSA_2048_u128::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let rsa = RSA_4096_u128::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let rsa = RSA_1024_u64::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let rsa = RSA_2048_u64::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let rsa = RSA_4096_u64::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let rsa = RSA_1024_u32::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let rsa = RSA_2048_u32::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let rsa = RSA_4096_u32::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let rsa = RSA_1024_u16::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let rsa = RSA_2048_u16::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let rsa = RSA_4096_u16::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let rsa = RSA_1024_u8::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulo);
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let rsa = RSA_2048_u8::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulo);
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let rsa = RSA_4096_u8::new_with_prepared_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulo = rsa.get_modulo();
        println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulo);
        println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulo);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}
