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
    // rsa_quick_start();
    // rsa_new();
    // rsa_new_with_automatic_keys();
    // rsa_new_with_keys();
    // rsa_new_with_primes();
    // rsa_new_with_prepared_keys();
    // rsa_set_keys();
    // rsa_find_keys();
    // calculate_keys();
    // rsa_encrypt_biguint();
    // rsa_decrypt_biguint();
    // rsa_failure_decrypt_biguint();
    // rsa_decrypt_array_biguint();
    // rsa_decrypt_unit();
    rsa_decrypt_array_unit();
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
        let modulus = rsa.get_modulus();
        println!("RSA_1024: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let rsa = RSA_2048::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let rsa = RSA_4096::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let rsa = RSA_Generic::<8, u32, 5>::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let rsa = RSA_1024_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let rsa = RSA_2048_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let rsa = RSA_4096_u128::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let rsa = RSA_1024_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let rsa = RSA_2048_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let rsa = RSA_4096_u64::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let rsa = RSA_1024_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let rsa = RSA_2048_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let rsa = RSA_4096_u32::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let rsa = RSA_1024_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let rsa = RSA_2048_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let rsa = RSA_4096_u16::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let rsa = RSA_1024_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let rsa = RSA_2048_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let rsa = RSA_4096_u8::new_with_automatic_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulus);
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
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024::new_with_keys(public_key, private_key, modulus);
    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048::new_with_keys(public_key, private_key, modulus);
    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096::new_with_keys(public_key, private_key, modulus);
    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let private_key = U512::from_str_radix("46CAC62F340DCC24A6FD1C603B57B4EE361C7C4370B01D769E53BC74CABC6153", 16).unwrap();
    let public_key = U512::from(3_u8);
    let modulus = U512::from_str_radix("6a302946ce14b236fa7baa9059038f669cf8c4f1988fe647d4583bd600fecf6d", 16).unwrap();

    let rsa = RSA_Generic::<16, u32, 5>::new_with_keys(public_key, private_key, modulus);
    println!("RSA_Generic: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let public_key = U1024::from(7_u8);
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();

    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let public_key = U2048::from(3_u8);
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();

    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let public_key = U4096::from(5_u8);
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();

    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("-------------------------------");
}

fn rsa_new_with_primes()
{
    println!("rsa_new_with_primes");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let prime1 = U512::from_str_radix("83CD305566CAC6013CC9A4230B040C64CFA7E50948208099BFB2744DD0DE8307", 16).unwrap();
        let prime2 = U512::from_str_radix("EA2A0C0B9696A6249323A66E5B4C2A8DD2E3417D3F9B3F48E603AD85943547DF", 16).unwrap();

        let rsa = RSA_Generic::<32, u16, 5>::new_with_primes(prime1, prime2);
        println!("RSA_Generic: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_Generic: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u128::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u128::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u128::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u64::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u64::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u64::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u32::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u32::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u32::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u16::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u16::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u16::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
        let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();

        let rsa = RSA_1024_u8::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let prime1 = U1024::from_str_radix("FFA2F7E4403DB8E0E647FACC4B514D4D7F8108EE7F7AF0ED6796A2BE34F535B81F229AE4765B5773478F40BED7E78CEC7181344512D11497C68B2F5EC3E66EA9", 16).unwrap();
        let prime2 = U1024::from_str_radix("DD94ECE66B19E7201B165E79C6A63F5FAF1207A3F75EECB1CAF944CB06F47D4610163F46035834981B419A64AAD2C39575F4D10FBFF01DFAFAB3C1106E2D0E97", 16).unwrap();

        let rsa = RSA_2048_u8::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let prime1 = U2048::from_str_radix("9EEA9259F72AB061072A91DD5BB8A5C8C0ED450C427A235B3123F331DB9229C826AD0440E3CA467B1A139134224574133594CBDB74A14A79BBDB148F56405BAFAD66B9912B492928BD0568E3E1530D17164C30AD1EC791B835DCE536233A76405E249B1B735A22892F16B670B5A61007814FF79A85AA69E6251F9BED3CE8170B", 16).unwrap();
        let prime2 = U2048::from_str_radix("94A16F977E8532F43A8A87C2EDC63A8B6D9930F99AA80EE1BC0E38C088B5128EC6539B32420D513757E16D41C338146A13A9107730BCC7C53CA73BC5FC2D9D3685806610C8602158BA28DF64AC0D27A66195EEC59FB424B21DA52D0A5B78034299C7CF14207175C856AC0171BA0AADDA6B75651554627C8A4BD028F43E81DC8F", 16).unwrap();

        let rsa = RSA_4096_u8::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn rsa_new_with_prepared_keys()
{
    println!("rsa_new_with_prepared_keys");
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let rsa = RSA_1024::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let rsa = RSA_2048::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let rsa = RSA_4096::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let rsa = RSA_Generic::<8, u32, 5>::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let rsa = RSA_1024_u128::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let rsa = RSA_2048_u128::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let rsa = RSA_4096_u128::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let rsa = RSA_1024_u64::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let rsa = RSA_2048_u64::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let rsa = RSA_4096_u64::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let rsa = RSA_1024_u32::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let rsa = RSA_2048_u32::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let rsa = RSA_4096_u32::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let rsa = RSA_1024_u16::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let rsa = RSA_2048_u16::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let rsa = RSA_4096_u16::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let rsa = RSA_1024_u8::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let rsa = RSA_2048_u8::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let rsa = RSA_4096_u8::new_with_prepared_keys();
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulus);
    println!("-------------------------------");
}

fn rsa_set_keys()
{
    println!("rsa_find_keys");
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let mut rsa = RSA_1024::new();
    rsa.set_public_key(U1024::from(7_u8));
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024: public key = {:X}:{:x}", public_key, modulus);

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let mut rsa = RSA_2048::new();
    rsa.set_public_key(U2048::from(7_u8));
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let mut rsa = RSA_4096::new();
    rsa.set_public_key(U4096::from(5_u8));
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let mut rsa = RSA_Generic::<8, u32, 5>::new();
    rsa.set_public_key(BigUInt::<u32, 8>::from(3_u8));
    rsa.set_private_key(BigUInt::<u32, 8>::from_str_radix("46CAC62F340DCC24A6FD1C603B57B4EE361C7C4370B01D769E53BC74CABC6153", 16).unwrap());
    rsa.set_modulus(BigUInt::<u32, 8>::from_str_radix("6a302946ce14b236fa7baa9059038f669cf8c4f1988fe647d4583bd600fecf6d", 16).unwrap());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let mut rsa = RSA_1024_u128::new();
    rsa.set_public_key(U1024::from(7_u8).into_biguint());
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap().into_biguint());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let mut rsa = RSA_2048_u128::new();
    rsa.set_public_key(U2048::from(7_u8).into_biguint());
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap().into_biguint());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096_u128;

    let mut rsa = RSA_4096_u128::new();
    rsa.set_public_key(U4096::from(5_u8).into_biguint());
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap().into_biguint());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let mut rsa = RSA_1024_u64::new();
    rsa.set_public_key(U1024::from(7_u8).into_biguint());
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap().into_biguint());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let mut rsa = RSA_2048_u64::new();
    rsa.set_public_key(U2048::from(7_u8).into_biguint());
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap().into_biguint());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let mut rsa = RSA_4096_u64::new();
    rsa.set_public_key(U4096::from(5_u8).into_biguint());
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap().into_biguint());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let mut rsa = RSA_1024_u32::new();
    rsa.set_public_key(U1024::from(7_u8).into_biguint());
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap().into_biguint());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let mut rsa = RSA_2048_u32::new();
    rsa.set_public_key(U2048::from(7_u8).into_biguint());
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap().into_biguint());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let mut rsa = RSA_4096_u32::new();
    rsa.set_public_key(U4096::from(5_u8).into_biguint());
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap().into_biguint());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let mut rsa = RSA_1024_u16::new();
    rsa.set_public_key(U1024::from(7_u8).into_biguint());
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap().into_biguint());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let mut rsa = RSA_2048_u16::new();
    rsa.set_public_key(U2048::from(7_u8).into_biguint());
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap().into_biguint());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let mut rsa = RSA_4096_u16::new();
    rsa.set_public_key(U4096::from(5_u8).into_biguint());
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap().into_biguint());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let mut rsa = RSA_1024_u8::new();
    rsa.set_public_key(U1024::from(7_u8).into_biguint());
    rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap().into_biguint());
    rsa.set_modulus(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let mut rsa = RSA_2048_u8::new();
    rsa.set_public_key(U2048::from(7_u8).into_biguint());
    rsa.set_private_key(U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap().into_biguint());
    rsa.set_modulus(U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulus);
    
    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let mut rsa = RSA_4096_u8::new();
    rsa.set_public_key(U4096::from(5_u8).into_biguint());
    rsa.set_private_key(U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap().into_biguint());
    rsa.set_modulus(U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap().into_biguint());
    let private_key = rsa.get_private_key();
    let public_key = rsa.get_public_key();
    let modulus = rsa.get_modulus();
    println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulus);
    println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulus);
    println!("-------------------------------");
}

fn rsa_find_keys()
{
    println!("rsa_find_keys");

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024;

        let mut rsa = RSA_1024::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let mut rsa = RSA_2048::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let mut rsa = RSA_4096::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let mut rsa = RSA_Generic::<8, u32, 5>::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_Generic<8, u32, 5>: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_Generic<8, u32, 5>: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let mut rsa = RSA_1024_u128::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u128: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let mut rsa = RSA_2048_u128::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u128: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let mut rsa = RSA_4096_u128::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u128: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u128: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let mut rsa = RSA_1024_u64::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u64: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let mut rsa = RSA_2048_u64::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u64: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let mut rsa = RSA_4096_u64::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u64: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u64: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let mut rsa = RSA_1024_u32::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u32: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let mut rsa = RSA_2048_u32::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u32: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let mut rsa = RSA_4096_u32::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u32: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u32: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let mut rsa = RSA_1024_u16::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u16: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let mut rsa = RSA_2048_u16::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u16: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let mut rsa = RSA_4096_u16::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u16: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u16: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let mut rsa = RSA_1024_u8::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_1024_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_1024_u8: public key = {:X}:{:x}", public_key, modulus);
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let mut rsa = RSA_2048_u8::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_2048_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_2048_u8: public key = {:X}:{:x}", public_key, modulus);
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let mut rsa = RSA_4096_u8::new();
        rsa.find_keys();
        let private_key = rsa.get_private_key();
        let public_key = rsa.get_public_key();
        let modulus = rsa.get_modulus();
        println!("RSA_4096_u8: private key = {:X}:{:x}", private_key, modulus);
        println!("RSA_4096_u8: public key = {:X}:{:x}", public_key, modulus);
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn calculate_keys()
{
    println!("calculate_keys");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut thread = Vec::<fn()>::new();
    
    // Example for RSA_1024
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024;

        let mut rsa = RSA_1024::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048;

        let mut rsa = RSA_2048::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096;

        let mut rsa = RSA_4096::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_Genric
    thread.push(||{
        use cryptocol::asymmetric::RSA_Generic;

        let mut rsa = RSA_Generic::<4, u128, 5>::new();
        let prime1 = U512::from_str_radix("F294849D5E617CC84E12A5F7E27B9D9E7DB99AE139694062D50C3243BC0DCC03", 16).unwrap();
        let prime2 = U512::from_str_radix("97A3CE282F1D1497870F3120D24216F9EE7BA23B50326416A7A67EEB250E8579", 16).unwrap();

        rsa.calculate_keys(prime1, prime2);
        println!("RSA_Generic: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_Generic: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u128;

        let mut rsa = RSA_1024_u128::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u128;

        let mut rsa = RSA_2048_u128::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u128
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u128;

        let mut rsa = RSA_4096_u128::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u64;

        let mut rsa = RSA_1024_u64::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u64;

        let mut rsa = RSA_2048_u64::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u64
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u64;

        let mut rsa = RSA_4096_u64::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u32;

        let mut rsa = RSA_1024_u32::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u32;

        let mut rsa = RSA_2048_u32::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u32
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u32;

        let mut rsa = RSA_4096_u32::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u16;

        let mut rsa = RSA_1024_u16::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u16;

        let mut rsa = RSA_2048_u16::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u16
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u16;

        let mut rsa = RSA_4096_u16::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_1024_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_1024_u8;

        let mut rsa = RSA_1024_u8::new();
        let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
        let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    // Example for RSA_2048_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_2048_u8;

        let mut rsa = RSA_2048_u8::new();
        let prime1 = U1024::from_str_radix("AD302C104ACE9E6BF8ADACC4BD0F3FDF141BB9B0A7285E9948B009F5A276F1D3F22C84199CB84D94BB41AD4BF2A9173D9A26132174E766F963F9655281B27AC265C17AEE34BEF077CED4F1EE13428DEF58AD5184D37C4169FDF3D922B678E65C4D8C64A870607C869C6328B790F4418D7425A0A92E169CB3609D28B5CC647D2D", 16).unwrap();
        let prime2 = U1024::from_str_radix("91B307B767F9BDE99AB167625F41A0396E3B06CC6C1059E5EACF42342013C99EBE230B7AD380577521F242E3A84AD15B2F7B96F95B5C35FA409FE60410457A8DB1CBF45D2592509DFA83A4B4EDBEE35075B79EA36B4B99B5F88F5AEC40CB0CD5C1B2D9853EA830CF08FAE6E5232F47F5BCE755AC8F3A0DFD861F1124BE3457C9", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });
    
    // Example for RSA_4096_u8
    thread.push(||{
        use cryptocol::asymmetric::RSA_4096_u8;

        let mut rsa = RSA_4096_u8::new();
        let prime1 = U2048::from_str_radix("E12DC76D28508F586A9C5812D8B17C9CE12AE362AB83E61D2BAB87B5C4C479E11A4429EEB3E72A8505F63F8DE53EF128C60DF09C2A301EFFD54A3D820F5DDC8A523AD6A73ED3DBFB8B3CE060FDCDEA86179EA426DFA7EFF2916848A4AF76E102A4152964FD49CEB0E7EA824CD6362E1EF2E10290B8D0DE717776C4100AD5150100E35F0F7DAA34B983A462DE0A2243659856A2C2B6A321E3FDFA3DFC0B5F6F027839300BA97A4FDE103DC0AE936D8263E9E9825959AE643E3DD2D44130EEA2E879D8EAA3852A740E65DE824ABF39FAEE7A75E08A5AB53C7F65A0DA865638C8DCE407E1BA5441E2223F0654848D0FB2689A99FF29E9A73C4F6658BF6657DD275F", 16).unwrap();
        let prime2 = U2048::from_str_radix("D29FC14636D944F57043C63461D40846452C3211654B3C68F77C1C8F28D8E4E323F27779B1F4F6BC2C3D245F77D18CDEEAAB8FCA778991608D6A10D1C7A1F36F2B143EF89A463AC998098116E7024790DE0FA381120986FEC53A659AC6F0D2B1D7EAC64466F6F62CA409B683DF9882980DE906BC1CC105C60134841D29356F731A6ABB832215AEE148D9E1DE35EAB9E61A289C1EC811F615502954088BDFAFDFF9902505E41EB4C896727D1032D795B1B315104536F8F9EE6757360E5E3E4B80D1E62D355C4DB2C59C9F8B66CADF0781C3D9211E71471CB93F79489EDDDF5C1E546ADA8EB42BF27960E4EB75EDD90A3B3A32C815A6879886A82D107E150587CD", 16).unwrap();

        rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
        println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
        println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    });

    do_simultaneously(thread);
    println!("-------------------------------");
}

fn rsa_encrypt_biguint()
{
    println!("rsa_encrypt_biguint");
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let public_key = BigUInt::<u64, 4>::from(5_u8);
    let private_key = BigUInt::<u64, 4>::from_str_radix("56C852580B06FD2C819A2A2AB58017266A58A101105A4F2FF0A9D17BB746239D", 16).unwrap();
    let modulus = BigUInt::<u64, 4>::from_str_radix("90a333e81260fb4a2d5646472e802696ddd96c20e9c0faae3db46db413927641", 16).unwrap();
    let rsa = RSA_Generic::<4, u64, 5>::new_with_keys(public_key, private_key, modulus);
    let message = BigUInt::<u64, 4>::from_string("55555555555555555555555555555555555555555555555").unwrap();

    println!("RSA_Generic<4, u64, 5>: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message);
    println!("RSA_Generic<4, u64, 5>: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "17457503729755083380415471913707555924530359885949510867590691646758707405421");

    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    let modulus = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let public_key = U2048::from(3_u8);
    let private_key = U2048::from_str_radix("701F1A22CF4EC7F8084FCE973A11B7C769D977C65E2CE023C8680270882A63AA5A1F591DBB7D9250CFE60485E22076BBF7112339393D3BB9E55A44B7584EF4A77C74040796B78464BF1E636A4033725D89B667CE27682731F2DA5A77662A7C39AF47E0078D9CFD8398DC67C20237758AE3E4CC3EE9A6364A5A70D6A886F7C5AD782D8E9700C462DC5550E884501351BF7CA151550F46724324548F2E83B7D1832A0F04B30314C14AE9FCE01E1D91B6ACF4EDAAADCD17298924B0B687DA02028B26F90A817EDFA1773396FDE0EA61A2E86C8ED519A38910F6974ABA4DDEEEF3F3D6420A62A6A7038B4DAA8D3CEB868FF4D79C2B0BF5B9D4E8A0F373AB963B3A3B", 16).unwrap();
    let modulus = U2048::from_str_radix("a82ea73436f62bf40c77b5e2d71a93ab1ec633a98d435035ac9c03a8cc3f957f872f05ac993c5b7937d906c8d330b219f299b4d5d5dbd996d807671304766efb3aae060b621346971ead951f604d2b8c4e919bb53b1c3acaec4787b3193fba5686ebd00b546b7c45654a9ba30353305055d7325e5e79516f87a941fcca73a885da9d96da9a86019b32e68bdca7737d4d24de47271efc65157108665b368414bb458c5646dde153694c78ee6e7a767e3d9f28801508dfb50c63a9aba588ca8c3db6d331754692ef56ffcb442649d207c50e4f860e62b4615bc15047f6d6f058ddda1788f43db6bcfe30b948ccb328a6374c24a9d6779f5e3f2e72b4d17795b39f", 16).unwrap();
    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "11826648665573892988392569847230424789544841077331225957148399871249268927154623470993762522900445434965023639973787631496464777431420694899592236563565553447397524662288366787064121938211894260367352462080016370592189375621911613195326192686816328076467380983986600488501826732392039921304723843353561979971845731242251071539539421583735299628338836052636445230864534605339617629402800559440180501290669969356432646609355691679827096191085790246438685484796134722878156083210208240925495334987760220950875141064570439965872317052753944644376506387327199045809671339203409748804852257989296314879619587232751200838815");

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");
    println!("-------------------------------");
}

fn rsa_decrypt_biguint()
{
    println!("rsa_decrypt_biguint");
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "1291365925956990209912301096206436401667136505494483326552892388697672421035786871295122243785492455365813633120609856123229759376007949983288196921350326580260085264651231909714016439201273309984156600534662061934819677352804108186913796503115103904740272641330383917375028485736726326961575966018344843544375111956896856459849745008491312504103267740859806938072560491882604981955826775714585411069204539358512084100532931709899505686043333904082157271127996483954201497588950610925938893405816546812856533512146469224967713898475743705008672515037488074806072422136954510678053632668629532542247787428389031452783");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let public_key = BigUInt::<u64, 4>::from(0xD_u8);
    let private_key = BigUInt::<u64, 4>::from_str_radix("3F3597F1C44073A8D3C34F5FF1444665DAE8D8F268104A60C82825E1C3CD44D5", 16).unwrap();
    let modulus = BigUInt::<u64, 4>::from_str_radix("88f41e8bd3e0fa98757c814fe013edde379c41da169a91050fa4964a1141853b", 16).unwrap();
    let rsa = RSA_Generic::<4, u64, 5>::new_with_keys(public_key, private_key, modulus);
    let message = BigUInt::<u64, 4>::from_string("55555555555555555555555555555555555555555555555").unwrap();

    println!("RSA_Generic<4, u64, 5>: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message);
    println!("RSA_Generic<4, u64, 5>: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "10674075484433295558572743933153491360572720800511665669089065789088032503360");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_Generic<4, u64, 5>: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024_u128: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "1291365925956990209912301096206436401667136505494483326552892388697672421035786871295122243785492455365813633120609856123229759376007949983288196921350326580260085264651231909714016439201273309984156600534662061934819677352804108186913796503115103904740272641330383917375028485736726326961575966018344843544375111956896856459849745008491312504103267740859806938072560491882604981955826775714585411069204539358512084100532931709899505686043333904082157271127996483954201497588950610925938893405816546812856533512146469224967713898475743705008672515037488074806072422136954510678053632668629532542247787428389031452783");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048_u128: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u128: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u128: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096_u128: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024_u64: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "1291365925956990209912301096206436401667136505494483326552892388697672421035786871295122243785492455365813633120609856123229759376007949983288196921350326580260085264651231909714016439201273309984156600534662061934819677352804108186913796503115103904740272641330383917375028485736726326961575966018344843544375111956896856459849745008491312504103267740859806938072560491882604981955826775714585411069204539358512084100532931709899505686043333904082157271127996483954201497588950610925938893405816546812856533512146469224967713898475743705008672515037488074806072422136954510678053632668629532542247787428389031452783");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048_u64: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u64: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u64: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096_u64: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    let modulus = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();
    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "99230296523159386700256526777425413629264442437576685721088524012075851202553740850680904664974658403571089264333361406651108391992097105729966644759559826283877160366748401717435051091667703209601257183162482151988604327001399793489756973079763602326808546529246854475734240062792376394092317599015107711947");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024_u32: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("67EA9B746A1CD56D22A34A4B4CA0902596D39C7F7B88B2726B7757C1F82628F1F15B51F9A51B22CB932F1CFE59B34178CD6ED42571006CCECC23D2050D3189D4A9175E8BC1DFE77C55EF68E3036BED29BFA70CB76D51D74E54D3535B72CC676899C5FC60021929B939F57CFF92A471A48BFA806FDA2CF942515BC9012AAAE479303614EB9BA52EC8A76CA78AAEA820EC8214AADB70E12492C66060384B609F56395532879C06FA33D7242E55D30AD79BE7D6F8E97152405ABC7097AF6DBEF5B7D2DCD1D414BB29E387327025D66F6D32142C858DC33BC6861A7A95AF46431FDB6AD3B3108E7AA0387CCF99504DC882E2FB2E127CB981123421A629908EC4570B", 16).unwrap();
    let modulus = U2048::from_str_radix("9bdfe92e9f2b4023b3f4ef70f2f0d838623d6abf394d0baba13303a2f4393d6aea08faf677a8b4315cc6ab7d868ce23534263e382980a3363235bb0793ca4ebefda30dd1a2cfdb3a80e71d548521e3be9f7a931323fac2f57f3cfd092c329b1ce6a8fa900325be95d6f03b7f5bf6aa76d1f7c0a7c74375e37a09ad81c00056b7637cc194d65b80229e6fa2fa18106eb682877e4763bdf6aa0315332a8709dd120b407a9e5d48f90b0b361b8b5c85ca5c2578aa01abdf00361fa3fca45a180b3211abaae3642188c455af8147f7cf31ac1cbb9532aaabbaecb40eb96c24bea9bc0c8ca36d83a25d73f14b13bf45de7e6483c246de199f10d89bad30b81a0f887b", 16).unwrap();
    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "6180106008116400276245342989473103484764761872267721473950722696146864035043160806694184669077268275204061759540743444182870700294283652408790531856247230367206621614772324283033339732101308701625462278598358955098096071549412033247370175467126791060041591140852809756204638561991812268754504459016425658025047148020085334540544993003178022240015169047284794069056176504629864457506720140976699242277352908155246616754908811577987276741973732843633335393961024533898633724469015271255095085368975909406244117060091735137408760315631597584694291792267983563546529313740251663261748403753363958654232541375089577621616");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048_u32: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("16D9A2A76977853F423E2DDBA4AECBA372B95E082577329BCA5852F4400907D2B34366C1DD35B01EE9A2483E823E635EB7492797BE652252E131EADD6A68CCFA952B49A7F09B9AB2653386B2DEEF6874F2724392C4544BEF7D18E83E7CEF59E21F550C23CE848A05A19DE2D97347946E86F734FCA112938024608A9C5C1D72D658F369B49D3A4C77ADB6483C632AAE94F345791F9313D304277B1E4275F72F7C9B7C2DAB7DA91B94C807E5553FE534A923D22206F6A987CB9214FCA3EB78CE22A434CBAF7C3DF29010688F7930E9EB2BBF6113DE6A35F78EFCFC9A17A868277BAD58F432F097405E3F9415FE4E15D11D8D5376747F6046DF74AE76C5B50D28C5D972724F09E84FD057DAC07625C06ACE068A171480D73F51CAF7728F9EE0F0D4F794F81B2AFCD7C15898C7C5B964130F0D4EA1819999CDBC8E000C500DF8FECCEB9EB8AA75C16EAA6C02C61A3129B49472136E1231BB9D563BFAF93B1322DF047F9FF7CDB74610AF49E7ADD790746C5DB31B1D48888764511AE10FF2F22BF4D1CBFE809A9C716D9C4043C59120BC5EA9CC0CA20C06D6DF11B0574E534757F9BC78617CB484C988871A065348DF542869140D0D0F58EB3A76E2E1394C21D8FFEEC315CFA03DC70269234EACFD8AEFDBCDF0C0EAE6C743135FFA2B504F937BD9F473F10862C2D59C38FAAEDB9E930EB9264723B90923B40056594592A16D7D0D9D", 16).unwrap();
    let modulus = U4096::from_str_radix("c72dbd530230cce40abeb30a9a33b60708cd8c4ef63091dcfe3f16ae76e1e10436e7004154dd34a85dbd0f0813e36b8ccc0fae90f5b5f3f20e915e9e276f6dd78f1eeb77f5016e7f6cdba95b31866f2ef6519dfb1e855acfa27f815134aaf70f3af39af5013a1971bc9558e476e9fc4ef2b0938fe0c42036f569879441cbe44ff664b937de05120c57823398d91e7cb389319fa8d6c50470ae7c83540b1e467d93a1d77a7e543845b3735aaa9b5a93543e286172b02ff34843c5d9075f0d2586754a4858c675641dc293d431fe5df11f3f8e7be802b8d566226b497e9cdcae7ce988ae6d8747771ebfb2eb235c58b9f08984c25336695731e5a13044139baf6a84137b163fb48e30d63809adc3853559e92df2420167d8eab2907724cbd33f5d712080a97195557b5e492c4472aba6e3dd9afe5ca37f2ccdff168d89e2ae6640935eb32f07fbb29663d320e65f14602974ac30124f1030f331db97f94471772e7a4a62074d517ea8c38bfac2492ca54bfa565c06e70263eab20c5f682d78b01a446a7a074b6ec5b14844fb28649eb9f746d7936ba191d9a7890a1f96bb3f4caa06a97383da413042e9c2b898f4397a1628d236607a0e0c1e4ef32a3d3d9ac59b0aab6632e85420be5de39ceb11b01c5fe3ee45d40418d8f21e894b292df523f570f96419ed5c3c0d0382b73ccec1f57976d790544d4ece8ebd145be1c130e827", 16).unwrap();
    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u32: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u32: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "374103778547092061962004288299115208376238320415950152369472097675929980153935401283925345495393450235206906720773160614616867222253137819577522888765833275561007893621096152233179171868886083460835425305117801157642091986349851413932705016493369007857870552243652087803382015694540662400578765207320399425048154146269581162065266583641548633891278741420175107121049522650466141596624367287590497133260408649273668727139433262055505123511164963693545573880831314381223431311081145504893254447162930451724660305799822259836914667838651280875559583209112473142570585350350615417988940095119386762044739541793817947934429176335870024385560095159386795033273614407369334003857709083012527420249947486898281210876194337051257649872331812976223032950323613439977681659861549325320693447666551984069525437252880368258280167539894135740654723284128996253012182985292335225328486391504159166101869356447735589666233694977344099218692858706630742326056159991482445749666805190877295673432692389657138426980184868183494023000749789723731848453658959105734519596827075324881824996533275662124645412829580548252866245777536895969022718423652423880459202825677905841662046792140571589175646463847633768079273417260356441398250086780413573340117792");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096_u32: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024_u16: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "1291365925956990209912301096206436401667136505494483326552892388697672421035786871295122243785492455365813633120609856123229759376007949983288196921350326580260085264651231909714016439201273309984156600534662061934819677352804108186913796503115103904740272641330383917375028485736726326961575966018344843544375111956896856459849745008491312504103267740859806938072560491882604981955826775714585411069204539358512084100532931709899505686043333904082157271127996483954201497588950610925938893405816546812856533512146469224967713898475743705008672515037488074806072422136954510678053632668629532542247787428389031452783");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048_u16: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u16: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u16: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096_u16: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();

    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_1024_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024_u8: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U2048::from_string("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();

    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_2048_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "1291365925956990209912301096206436401667136505494483326552892388697672421035786871295122243785492455365813633120609856123229759376007949983288196921350326580260085264651231909714016439201273309984156600534662061934819677352804108186913796503115103904740272641330383917375028485736726326961575966018344843544375111956896856459849745008491312504103267740859806938072560491882604981955826775714585411069204539358512084100532931709899505686043333904082157271127996483954201497588950610925938893405816546812856533512146469224967713898475743705008672515037488074806072422136954510678053632668629532542247787428389031452783");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_2048_u8: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = U4096::from_string("222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap();

    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u8: Message = {}", message);

    let cipher = rsa.encrypt_biguint(&message.into_biguint());
    println!("RSA_4096_u8: Cipher = {}", cipher);
    assert_eq!(cipher.to_string(), "306888388337466187282042307062704792652419569166337859961296761128522602783284171941314622434380020194218138738129324632968494397146631631568752167149366814328742864467558796468059168590183607048154820747334404657757306388818354135775667034077939707535729072254477687440326383413829184673485777354355593153355280000684319981409686640441919882399187403283001545508316655294832517432640860568986729894134362791743417711899828106270794047478495501030619345165383268394002700760718934742142793958715037000863729220391884324389876262174512194595764289919358407235812417445470095912470878283115657786217581619570857356292100179097109404807157489324367049867976992155866043576802494631549170761502725304142654782565627033872965724907731137654011229939774263227934571010082210703339879069398028070889488455929343451400703302388062601385983831754138909311641485104519487019661743420872116450935046792460451393530116569148009578799297083991851298476713329013797490297381294533747638243268187426309927081476470270764781262099702867717226556463217265238293512242449585754661504083025550479259607209149236813578913767448367431207086242762349794159228923175132222452502498030057987566531658182120131188295100537311716791771009234223633803366604959");

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_4096_u8: Recovered = {}", recovered);
    assert_eq!(recovered, message.into_biguint());
    println!("-------------------------------");
}

fn rsa_failure_decrypt_biguint()
{
    println!("rsa_failure_decrypt_biguint");
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.clone(), private_key.clone(), modulus.clone());

    // Example for Message > self.modulus
    println!("Message > self.modulus");
    let message = modulus.wrapping_add_uint(1_u8);
    let distorted = U1024::one();

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}", message);
    println!("RSA_1024: Distorted = {}", distorted);

    let cipher = rsa.encrypt_biguint(&message);
    println!("RSA_1024: Cipher = {}", cipher);
    let cypher = rsa.encrypt_biguint(&distorted);
    println!("RSA_1024: Cypher = {}", cypher);
    assert_eq!(cipher.to_string(), "1");
    assert_eq!(cypher.to_string(), "1");
    assert_eq!(cipher, cypher);

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024: Recovered = {}", recovered);
    assert_ne!(recovered, message);
    let back = rsa.decrypt_biguint(&cypher);
    println!("RSA_1024: Back = {}", back);
    assert_ne!(back, message);
    assert_eq!(back, distorted);
    println!();

    // Example for Message == self.modulus
    println!("Message == self.modulus");
    let message = modulus.clone();
    let distorted = U1024::zero();

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}", message);
    println!("RSA_1024: Distorted = {}", distorted);

    let cipher = rsa.encrypt_biguint(&message);
    println!("RSA_1024: Cipher = {}", cipher);
    let cypher = rsa.encrypt_biguint(&distorted);
    println!("RSA_1024: Cypher = {}", cypher);
    assert_eq!(cipher.to_string(), "0");
    assert_eq!(cypher.to_string(), "0");
    assert_eq!(cipher, cypher);

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024: Recovered = {}", recovered);
    assert_ne!(recovered, message);
    let back = rsa.decrypt_biguint(&cypher);
    println!("RSA_1024: Back = {}", back);
    assert_ne!(back, message);
    assert_eq!(back, distorted);
    println!();

    // Example for Message < self.modulus
    println!("Message < self.modulus");
    let message = modulus.wrapping_sub_uint(1_u8);
    let undistorted = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a8", 16).unwrap();

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}", message);
    println!("RSA_1024: Undistorted = {}", undistorted);

    let cipher = rsa.encrypt_biguint(&message);
    println!("RSA_1024: Cipher = {}", cipher);
    let cypher = rsa.encrypt_biguint(&undistorted);
    println!("RSA_1024: Cypher = {}", cypher);
    assert_eq!(cipher.to_string(), "107593610008203780612632479874283724612849929433287325474357706755534020189965489050824167197114399993039200600891832987535334998852663448632692843291036287305929545774864548523913837119097088255084765151875474264208848513096610739812512238075122468981229742553101403587095991215625906902923820467139749087400");
    assert_eq!(cypher.to_string(), "107593610008203780612632479874283724612849929433287325474357706755534020189965489050824167197114399993039200600891832987535334998852663448632692843291036287305929545774864548523913837119097088255084765151875474264208848513096610739812512238075122468981229742553101403587095991215625906902923820467139749087400");
    assert_eq!(cipher, cypher);

    let recovered = rsa.decrypt_biguint(&cipher);
    println!("RSA_1024: Recovered = {}", recovered);
    assert!(recovered == message);
    let back = rsa.decrypt_biguint(&cypher);
    println!("RSA_1024: Back = {}", back);
    assert!(back == message);
    assert!(back == undistorted);
    println!("-------------------------------");
}

fn rsa_decrypt_array_biguint()
{
    println!("rsa_decrypt_array_biguint");
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_1024: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_2048: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_4096: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let public_key = BigUInt::<u8, 32>::from(0xD_u8);
    let private_key = BigUInt::<u8, 32>::from_str_radix("3F3597F1C44073A8D3C34F5FF1444665DAE8D8F268104A60C82825E1C3CD44D5", 16).unwrap();
    let modulus = BigUInt::<u8, 32>::from_str_radix("88f41e8bd3e0fa98757c814fe013edde379c41da169a91050fa4964a1141853b", 16).unwrap();
    let rsa = RSA_Generic::<32, u8, 5>::new_with_keys(public_key, private_key, modulus);
    let message = [BigUInt::<u8, 32>::from_string("111111111111111111111111111111111111111111111111").unwrap(),
                    BigUInt::<u8, 32>::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    BigUInt::<u8, 32>::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_Generic::<32, u8, 5>: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic::<32, u8, 5>: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_Generic::<32, u8, 5>: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_Generic::<32, u8, 5>: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "51720491092447519507074683521365241726426520130311862375451081363728566503113");
    assert_eq!(cipher[1].to_string(), "23629251217471383540783659092109414281325061527006739986059075736051020827090");
    assert_eq!(cipher[2].to_string(), "43982897865412306355287000322072606109839286077564333327205434569348156907984");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_Generic::<32, u8, 5>: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    
    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u128: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_1024_u128: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024_u128: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u128: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_2048_u128: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048_u128: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u128: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_4096_u128: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096_u128: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    
    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u64: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_1024_u64: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024_u64: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u64: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_2048_u64: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048_u64: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u64: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_4096_u64: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096_u64: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    
    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u32: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_1024_u32: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024_u32: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u32: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_2048_u32: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048_u32: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u32: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_4096_u32: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096_u32: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    
    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u16: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_1024_u16: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024_u16: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u16: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_2048_u16: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048_u16: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u16: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    println!("RSA_4096_u16: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096_u16: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    
    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_1024_u8: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].clone(), message[1].clone(), message[2].clone()]);
    println!("RSA_1024_u8: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_1024_u8: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U2048::from_string("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U2048::from_string("22222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U2048::from_string("33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_2048_u8: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].clone(), message[1].clone(), message[2].clone()]);
    println!("RSA_2048_u8: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "19222624061359673250846359059739843853420634451448389991163689366334342051370657105636882731845894634918805671425622270152257383139670767434742531576389209205471554857350405398176392673609145359240099506218157452818844010262047559375358378002428435697834074559358972665165014579261334313265387506189805058027311828755496605900175422002860565918262405898991726046385746431718588466318613122627318460097165106135557737414837339967603607946756100252603428997909207374469669014045491975776570237713303509787189577741351064528943707348455591901381015422856571578745503309804988710845868795458723379943018626701026619580904");
    assert_eq!(cipher[1].to_string(), "2162601705197266388430407067468432039384013235712826322894856463766187033358723916690576103616208021235118086892704527157675665923187607693853175631923961500069870836826565615537684511580445390403335706835152446778962609842260014842938385695677464053437897083002342473290905366555201087408506088286024071657223370958139726526658613908350434381933078613256245009076381433448776545677230225375345485184663906559997314931625379179728845765913371880777924776671070365050266168791435461962831816750003484563263566103101048540393850384009497837363571269795809614594215419935841678869610883053048459763096376368303543057618");
    assert_eq!(cipher[2].to_string(), "11005496648948092196671263853474018345023369218256547760376027065480788176360278240016253379330891750962711210125748191463703823515643414126048153016279497771873322920552753976875286327495435794501104988281718167000843412300082848991944943057029810023543981865177927105338265914121416097511479737861674883541423933061998681507286171218723416306799646568348207996021600555324586523788204283964397436578478311759288379296955383037548813806708090848554801088192089524689822235188613606104977142437789920233100836515577891822546346199757457300078280962387025576467704769692869917952700145348667083921490956402329000033669");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_2048_u8: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [U4096::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
                    U4096::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
                    U4096::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];

    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    println!("RSA_4096_u8: Message = {}-{}-{}", message[0], message[1], message[2]);

    let cipher = rsa.encrypt_array_biguint(&[message[0].clone(), message[1].clone(), message[2].clone()]);
    println!("RSA_4096_u8: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    assert_eq!(cipher[0].to_string(), "482480559007706139120435280808635896009433629588335984384927715221286241221693386397964558645023125644387584377108048840919363748179067815556343522612009226536244646018841314849644563703567121309251931243104536248830409306764441218940707735488174389293011516384299039172989527209791652903206065174607465242127258473463449897381586375165158262679436131223964565287682877733440561251733628268289938957295135308390468247660146474573679954253402229925356718786306318712335967935544719625339452930957199350386947670968898258607047286750312571697576971474019840700563054069610571790835804735603849906670776683787524123334029659362618326992712141061257613414324296166006594978236006676084553601587664366122587386278129925060768375788485986602763782980169513776316287742616158875810163725010461579433100067994550624723427286094489426319062692258967640187754398048428613760189298914444204926481463733838943736719093910153478487926462136253769740147515107152461575428182350390686729098858398529816214835849111530826241152472252076472917684397765104370104272368489696053079777801388987696516172596260735578907952952459711577927570207745725289641141609304276024163405203474353382102713816313135307667770197490398260668967746483598882575518327870");
    assert_eq!(cipher[1].to_string(), "380120271426728247273769594446426641238838574667066484492143825438199540746259333734898815839113476807946344674781852498015750187211946722686817140378127692602596973237535781766036917217014439840298935296104792388923935389140007930758417023287106240377236524088212457735840517977835282307935378303691528124337422295542861244981251202817035532847360174307949160666481423592218164303960867236056117050624200788667329124296084136158402380660016117503455576500836950136532659050275799351693675963408739507550453651080705341452109450531835286543627282055447987805443778346211402384761807314384195838018252816026053966001141868460018268757844053788682677555437718750833050059731042394824540328543797043564501760327576307307355931319658681398516269273687097684141437367423611622209771531132413287920345638854815045079343370332832839457765574775484259542386378149003148326014739675786806590309375655758321160305411682476757937244509127945920548992083112247444836804339957942255918239364228741912091347528452773593332640033742044746411180874088216225474181184840821497671566688088032569517704444379135001484669795569619902101545075816910203604438757433894421722427813437974178973513107314677953578676777016943686788116194436463496787358425447");
    assert_eq!(cipher[2].to_string(), "403708122166717805212812440921541454803287378381881151718566217059934543482388343843574347969379105455056108349254666875616590987975539200841837849153148937087547866812511369840092694040775151853507412463054629015063201331777319928369501610686490431829222617691381034382645478686717780175687315400539711928636188622663308476599958801157371748300079694552482160722166897070092451616412069071181575031872050871255427968090224828478189746473554664127514715116670586818360179040472853498418648045879823744470657856376457526201599706788452275951271097481060745828920788111970622502535466247898449235637174171943825913108633880035293325538963948968316813709719634673738870061012948875675340444118678854664035378384510021029569760389132667739424199676333150729497633898277251965456673950972150625131498563419556536557513225945879539695884910650410670631396971797107947173943230798002349278205019698905650667265388286337528525078132653138115630367203064184429904843606583697672829370055741098908264941689234822140860264804737667847796895993509920072611623938284873822582637972665248640500313543359063164177031112994089881032048285300304361927726006864552178984319375585107683011215008616994395932187400440737780861175121938401991141425571085");

    let recovered = rsa.decrypt_array_biguint(&cipher);
    println!("RSA_4096_u8: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    assert_eq!(recovered[0], message[0].into_biguint());
    assert_eq!(recovered[1], message[1].into_biguint());
    assert_eq!(recovered[2], message[2].into_biguint());
    println!("-------------------------------");
}

fn rsa_decrypt_unit()
{
    println!("rsa_decrypt_unit");
    use std::ptr::copy_nonoverlapping;
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 32];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 32); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_1024: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x5876D910, 0x9DF1BA1, 0x3D2ABEA9, 0xF9EAF9F1, 0xB4DD00B4, 0x4238994, 0x946574F8, 0xFB00D2B7, 0xD3F9E91D, 0x26D78E8F, 0x4D1B93C, 0x7666C3CC, 0x492EA323, 0xC7EFA926, 0x95E9D5CE, 0xF32C4732, 0x748103D, 0x298576A7, 0x4342BA6D, 0xFF59D1A9, 0x9D1585DD, 0xFA9B236F, 0x7B7982E2, 0x2B80425C, 0x1112A9DA, 0xA5EA8BC7, 0x41AF7FBB, 0x557B6333, 0xFA12D78B, 0x34B8451C, 0x867DF90A, 0xB412E61E]);

    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 8];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rr.as_mut_ptr() as *mut u32, 32); }
    
    print!("RSA_1024: Recovered = [");
    for r in rr
        { println!("{:#X}, ", r); }
    assert_eq!(rr, [0x_123456789ABCDEF00FEDCBA987654321, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 64];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 64); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_2048: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x47283873, 0x2CDBEDD8, 0x93922441, 0x8E8A2C0F, 0xD2483CDF, 0x712502C, 0x975FB948, 0xA5477C70, 0x5151E7A4, 0x7B83A1E1, 0x63B4157F, 0x9CB225EB, 0xF613588C, 0xA28B6E71, 0xD16F7440, 0xE80799C, 0xE5629F2E, 0x59A4DD76, 0x5CC7AF66, 0x17FD5732, 0x90E31987, 0x23B5BDDD, 0x2A329668, 0x91DA607E, 0xE2EB9B3B, 0x1DFC378F, 0x56EFB8F6, 0xA387D998, 0x7C819286, 0x7FFD1AC3, 0x81E4143F, 0x72B12533, 0x6C871DC6, 0x3317DDA8, 0x415F5497, 0x2FB59FFD, 0xF720E361, 0x515DB9B5, 0x8EB0C7D1, 0x2747A272, 0x661AB141, 0xFA4561A, 0x8E9DAD98, 0xC3DC9098, 0x6FC2E30, 0x6CCC919D, 0x44BCC6F4, 0xCA835FD1, 0xD660A677, 0xFC235E1F, 0x2D54CE65, 0x16AD846F, 0xB8FBDBC3, 0x5EBF02E, 0x63EE29D6, 0x49262F3B, 0x96CC8F26, 0x4BCD700C, 0x3362037A, 0xD909B057, 0x2D932D0D, 0x1394BCCB, 0xE545D5DF, 0x6A33D012]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 16];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rr.as_mut_ptr() as *mut u32, 64); }
    
    print!("RSA_2048: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x_123456789ABCDEF00FEDCBA987654321, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 128];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 128); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_4096: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x9CB6ED19, 0xD4FB55D8, 0xC122734C, 0x2F1811F6, 0xA5114AFF, 0x93B93341, 0x6CEDD077, 0xCBABA04C, 0xB0873367, 0xDDBB2719, 0x8060C197, 0x17949273, 0x9EBA2656, 0xA6757910, 0xF832F645, 0x12836967, 0x2ED8B7ED, 0x501E8879, 0x920F899A, 0x6AEBB339, 0xB416EC59, 0xDCB94159, 0xD24DF92D, 0x2457DBC7, 0xBF6C9361, 0x6308ADE8, 0xB7E0A22C, 0xF8641498, 0x20367D0C, 0x994F70D2, 0x2E1B1007, 0x96EA933B, 0x65D11AD3, 0x78C0C565, 0xB2E327C9, 0xC9A3DB5, 0x2A5DA924, 0xFFBB3522, 0xA6187825, 0x14F47769, 0x3E3E549A, 0xDCBCE40A, 0x27323317, 0xECC35120, 0xAEEB915, 0xC47F38FD, 0x6A9DEF00, 0x2D7AC41D, 0x9E9EBA3C, 0xB5347E7, 0xE9F682DF, 0x936934BD, 0x715BC0D4, 0x7709F812, 0x44E0BDF7, 0x3BD64D43, 0xCCD014E8, 0x54F45FBE, 0x2E2A4FEE, 0x71FD352A, 0x527F1386, 0x81683594, 0x5370C3FF, 0xE19A6E62, 0x39978561, 0x4AAB76D6, 0x3A41D681, 0xB5EE9ED7, 0x465EA91C, 0xEF7222B1, 0x8098DF3C, 0x975E545D, 0xDE5D873A, 0x2FD8CF10, 0xFF806337, 0xDF0246B0, 0x12ABF503, 0xB185DA9D, 0xE1AD50CB, 0x746198FC, 0x86727129, 0x48A7C5FE, 0xFCA33C4C, 0xC1B4D550, 0xF2FC1DFE, 0x13DE163B, 0x7E9624D7, 0x45C84418, 0xCF8DEA12, 0xB7F566C7, 0x46CFC115, 0x81E6CDC4, 0xD8BCFF70, 0xB41D8E6C, 0x9F7ABF3F, 0x3FCD96B, 0x40203895, 0xDBE711C6, 0xEDDB977A, 0x4F0F98A1, 0xE4060DC6, 0x34AC5734, 0x884CDEB5, 0x860425FA, 0xB9D26FF9, 0x89E2B6CF, 0x94351640, 0xC2F3797, 0xC944A9CC, 0xC5570B76, 0x15CF86D7, 0x5455267A, 0xDF827B3, 0x89C4D05D, 0xD3C161C5, 0x71F3A803, 0x275ECEA5, 0xCE1A255, 0x4B055080, 0x6E115E15, 0x1D64B0AA, 0xF1A92FD5, 0x9A1B3206, 0x37FC92CA, 0x62EE5AFA, 0xD18E769A, 0xF5CCF050, 0xEC82A614]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 32];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rr.as_mut_ptr() as *mut u32, 128); }
    
    print!("RSA_4096: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x_123456789ABCDEF00FEDCBA987654321, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]);
    assert_eq!(rr, message);

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let public_key = BigUInt::<u64, 4>::from(0xD_u8);
    let private_key = BigUInt::<u64, 4>::from_str_radix("3F3597F1C44073A8D3C34F5FF1444665DAE8D8F268104A60C82825E1C3CD44D5", 16).unwrap();
    let modulus = BigUInt::<u64, 4>::from_str_radix("88f41e8bd3e0fa98757c814fe013edde379c41da169a91050fa4964a1141853b", 16).unwrap();
    let rsa = RSA_Generic::<4, u64, 5>::new_with_keys(public_key, private_key, modulus);
    let message = [0x_FEDCBA9876543210_u64, 0x_1122334455667788, 0x_9900AABBCCDDEEFF, 0x_FEDCBA0987654321];

    println!("RSA_Generic<4, u64, 5>: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_Generic<4, u64, 5>: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let cipher = rsa.encrypt_unit(&message);
    print!("RSA_Generic<4, u64, 5>: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0xAA75AA5E6838BB51, 0x14B74CBA736261B8, 0x582AED04672A4512, 0xE81777B9B19C8547]);
    let recovered = rsa.decrypt_unit(&cipher);
    print!("RSA_Generic<4, u64, 5>: Recovered = [");
    for r in recovered
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(recovered, [0x_FEDCBA9876543210, 0x_1122334455667788, 0x_9900AABBCCDDEEFF, 0x_FEDCBA0987654321]);
    assert_eq!(recovered, message);

    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u128: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let cipher = rsa.encrypt_unit(&message);
    print!("RSA_1024_u128: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0xF9EAF9F13D2ABEA909DF1BA15876D910, 0xFB00D2B7946574F804238994B4DD00B4, 0x7666C3CC04D1B93C26D78E8FD3F9E91D, 0xF32C473295E9D5CEC7EFA926492EA323, 0xFF59D1A94342BA6D298576A70748103D, 0x2B80425C7B7982E2FA9B236F9D1585DD, 0x557B633341AF7FBBA5EA8BC71112A9DA, 0xB412E61E867DF90A34B8451CFA12D78B]);
    let recovered = rsa.decrypt_unit(&cipher);   

    print!("RSA_1024_u128: Recovered = [");
    for r in recovered
        { println!("{:#X}, ", r); }
    assert_eq!(recovered, [0x_123456789ABCDEF00FEDCBA987654321, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(recovered, message);

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u128: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let cipher = rsa.encrypt_unit(&message);
    print!("RSA_2048_u128: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x8E8A2C0F939224412CDBEDD847283873, 0xA5477C70975FB9480712502CD2483CDF, 0x9CB225EB63B4157F7B83A1E15151E7A4, 0xE80799CD16F7440A28B6E71F613588C, 0x17FD57325CC7AF6659A4DD76E5629F2E, 0x91DA607E2A32966823B5BDDD90E31987, 0xA387D99856EFB8F61DFC378FE2EB9B3B, 0x72B1253381E4143F7FFD1AC37C819286, 0x2FB59FFD415F54973317DDA86C871DC6, 0x2747A2728EB0C7D1515DB9B5F720E361, 0xC3DC90988E9DAD980FA4561A661AB141, 0xCA835FD144BCC6F46CCC919D06FC2E30, 0x16AD846F2D54CE65FC235E1FD660A677, 0x49262F3B63EE29D605EBF02EB8FBDBC3, 0xD909B0573362037A4BCD700C96CC8F26, 0x6A33D012E545D5DF1394BCCB2D932D0D]);
    let recovered = rsa.decrypt_unit(&cipher);

    print!("RSA_2048_u128: Recovered = [");
    for r in recovered
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(recovered, [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(recovered, message);

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u128: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let cipher = rsa.encrypt_unit(&message);
    print!("RSA_4096_u128: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x2F1811F6C122734CD4FB55D89CB6ED19, 0xCBABA04C6CEDD07793B93341A5114AFF, 0x179492738060C197DDBB2719B0873367, 0x12836967F832F645A67579109EBA2656, 0x6AEBB339920F899A501E88792ED8B7ED, 0x2457DBC7D24DF92DDCB94159B416EC59, 0xF8641498B7E0A22C6308ADE8BF6C9361, 0x96EA933B2E1B1007994F70D220367D0C, 0xC9A3DB5B2E327C978C0C56565D11AD3, 0x14F47769A6187825FFBB35222A5DA924, 0xECC3512027323317DCBCE40A3E3E549A, 0x2D7AC41D6A9DEF00C47F38FD0AEEB915, 0x936934BDE9F682DF0B5347E79E9EBA3C, 0x3BD64D4344E0BDF77709F812715BC0D4, 0x71FD352A2E2A4FEE54F45FBECCD014E8, 0xE19A6E625370C3FF81683594527F1386, 0xB5EE9ED73A41D6814AAB76D639978561, 0x975E545D8098DF3CEF7222B1465EA91C, 0xDF0246B0FF8063372FD8CF10DE5D873A, 0x746198FCE1AD50CBB185DA9D12ABF503, 0xC1B4D550FCA33C4C48A7C5FE86727129, 0x45C844187E9624D713DE163BF2FC1DFE, 0x81E6CDC446CFC115B7F566C7CF8DEA12, 0x3FCD96B9F7ABF3FB41D8E6CD8BCFF70, 0x4F0F98A1EDDB977ADBE711C640203895, 0x860425FA884CDEB534AC5734E4060DC6, 0xC2F37979435164089E2B6CFB9D26FF9, 0x5455267A15CF86D7C5570B76C944A9CC, 0x71F3A803D3C161C589C4D05D0DF827B3, 0x6E115E154B0550800CE1A255275ECEA5, 0x37FC92CA9A1B3206F1A92FD51D64B0AA, 0xEC82A614F5CCF050D18E769A62EE5AFA]);
    let recovered = rsa.decrypt_unit(&cipher);

    print!("RSA_4096_u128: Recovered = [");
    for r in recovered
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(recovered, [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]);
    assert_eq!(recovered, message);

    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u64: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u64; 16];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr(), 16); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_1024_u64: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x9DF1BA15876D910, 0xF9EAF9F13D2ABEA9, 0x4238994B4DD00B4, 0xFB00D2B7946574F8, 0x26D78E8FD3F9E91D, 0x7666C3CC04D1B93C, 0xC7EFA926492EA323, 0xF32C473295E9D5CE, 0x298576A70748103D, 0xFF59D1A94342BA6D, 0xFA9B236F9D1585DD, 0x2B80425C7B7982E2, 0xA5EA8BC71112A9DA, 0x557B633341AF7FBB, 0x34B8451CFA12D78B, 0xB412E61E867DF90A]);

    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 8];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u64, 16); }
    
    print!("RSA_1024_u64: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u64: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u64; 32];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr(), 32); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_2048_u64: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x2CDBEDD847283873, 0x8E8A2C0F93922441, 0x712502CD2483CDF, 0xA5477C70975FB948, 0x7B83A1E15151E7A4, 0x9CB225EB63B4157F, 0xA28B6E71F613588C, 0xE80799CD16F7440, 0x59A4DD76E5629F2E, 0x17FD57325CC7AF66, 0x23B5BDDD90E31987, 0x91DA607E2A329668, 0x1DFC378FE2EB9B3B, 0xA387D99856EFB8F6, 0x7FFD1AC37C819286, 0x72B1253381E4143F, 0x3317DDA86C871DC6, 0x2FB59FFD415F5497, 0x515DB9B5F720E361, 0x2747A2728EB0C7D1, 0xFA4561A661AB141, 0xC3DC90988E9DAD98, 0x6CCC919D06FC2E30, 0xCA835FD144BCC6F4, 0xFC235E1FD660A677, 0x16AD846F2D54CE65, 0x5EBF02EB8FBDBC3, 0x49262F3B63EE29D6, 0x4BCD700C96CC8F26, 0xD909B0573362037A, 0x1394BCCB2D932D0D, 0x6A33D012E545D5DF]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 16];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u64, 32); }
    
    print!("RSA_2048_u64: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u64: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u64; 64];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr(), 64); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_4096_u64: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");

    assert_eq!(cipher, [0xD4FB55D89CB6ED19, 0x2F1811F6C122734C, 0x93B93341A5114AFF, 0xCBABA04C6CEDD077, 0xDDBB2719B0873367, 0x179492738060C197, 0xA67579109EBA2656, 0x12836967F832F645, 0x501E88792ED8B7ED, 0x6AEBB339920F899A, 0xDCB94159B416EC59, 0x2457DBC7D24DF92D, 0x6308ADE8BF6C9361, 0xF8641498B7E0A22C, 0x994F70D220367D0C, 0x96EA933B2E1B1007, 0x78C0C56565D11AD3, 0xC9A3DB5B2E327C9, 0xFFBB35222A5DA924, 0x14F47769A6187825, 0xDCBCE40A3E3E549A, 0xECC3512027323317, 0xC47F38FD0AEEB915, 0x2D7AC41D6A9DEF00, 0xB5347E79E9EBA3C, 0x936934BDE9F682DF, 0x7709F812715BC0D4, 0x3BD64D4344E0BDF7, 0x54F45FBECCD014E8, 0x71FD352A2E2A4FEE, 0x81683594527F1386, 0xE19A6E625370C3FF, 0x4AAB76D639978561, 0xB5EE9ED73A41D681, 0xEF7222B1465EA91C, 0x975E545D8098DF3C, 0x2FD8CF10DE5D873A, 0xDF0246B0FF806337, 0xB185DA9D12ABF503, 0x746198FCE1AD50CB, 0x48A7C5FE86727129, 0xC1B4D550FCA33C4C, 0x13DE163BF2FC1DFE, 0x45C844187E9624D7, 0xB7F566C7CF8DEA12, 0x81E6CDC446CFC115, 0xB41D8E6CD8BCFF70, 0x3FCD96B9F7ABF3F, 0xDBE711C640203895, 0x4F0F98A1EDDB977A, 0x34AC5734E4060DC6, 0x860425FA884CDEB5, 0x89E2B6CFB9D26FF9, 0xC2F379794351640, 0xC5570B76C944A9CC, 0x5455267A15CF86D7, 0x89C4D05D0DF827B3, 0x71F3A803D3C161C5, 0xCE1A255275ECEA5, 0x6E115E154B055080, 0xF1A92FD51D64B0AA, 0x37FC92CA9A1B3206, 0xD18E769A62EE5AFA, 0xEC82A614F5CCF050]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 32];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u64, 64); }
    
    print!("RSA_4096_u64: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF, 0x11111111111111111111111111111111, 0x22222222222222222222222222222222, 0x33333333333333333333333333333333, 0x44444444444444444444444444444444, 0x55555555555555555555555555555555, 0x66666666666666666666666666666666, 0x77777777777777777777777777777777, 0x88888888888888888888888888888888, 0x99999999999999999999999999999999, 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0xCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0xDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x0]);
    assert_eq!(rr, message);

    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u32: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 32];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 32); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_1024_u32: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x5876D910, 0x9DF1BA1, 0x3D2ABEA9, 0xF9EAF9F1, 0xB4DD00B4, 0x4238994, 0x946574F8, 0xFB00D2B7, 0xD3F9E91D, 0x26D78E8F, 0x4D1B93C, 0x7666C3CC, 0x492EA323, 0xC7EFA926, 0x95E9D5CE, 0xF32C4732, 0x748103D, 0x298576A7, 0x4342BA6D, 0xFF59D1A9, 0x9D1585DD, 0xFA9B236F, 0x7B7982E2, 0x2B80425C, 0x1112A9DA, 0xA5EA8BC7, 0x41AF7FBB, 0x557B6333, 0xFA12D78B, 0x34B8451C, 0x867DF90A, 0xB412E61E]);

    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 8];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u32, 32); }
    
    print!("RSA_1024_u32: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u32: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 64];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 64); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_2048_u32: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x47283873, 0x2CDBEDD8, 0x93922441, 0x8E8A2C0F, 0xD2483CDF, 0x712502C, 0x975FB948, 0xA5477C70, 0x5151E7A4, 0x7B83A1E1, 0x63B4157F, 0x9CB225EB, 0xF613588C, 0xA28B6E71, 0xD16F7440, 0xE80799C, 0xE5629F2E, 0x59A4DD76, 0x5CC7AF66, 0x17FD5732, 0x90E31987, 0x23B5BDDD, 0x2A329668, 0x91DA607E, 0xE2EB9B3B, 0x1DFC378F, 0x56EFB8F6, 0xA387D998, 0x7C819286, 0x7FFD1AC3, 0x81E4143F, 0x72B12533, 0x6C871DC6, 0x3317DDA8, 0x415F5497, 0x2FB59FFD, 0xF720E361, 0x515DB9B5, 0x8EB0C7D1, 0x2747A272, 0x661AB141, 0xFA4561A, 0x8E9DAD98, 0xC3DC9098, 0x6FC2E30, 0x6CCC919D, 0x44BCC6F4, 0xCA835FD1, 0xD660A677, 0xFC235E1F, 0x2D54CE65, 0x16AD846F, 0xB8FBDBC3, 0x5EBF02E, 0x63EE29D6, 0x49262F3B, 0x96CC8F26, 0x4BCD700C, 0x3362037A, 0xD909B057, 0x2D932D0D, 0x1394BCCB, 0xE545D5DF, 0x6A33D012]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 16];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u32, 64); }
    
    print!("RSA_2048_u32: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u32: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u32; 128];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 128); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_4096_u32: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");

    assert_eq!(cipher, [0x9CB6ED19, 0xD4FB55D8, 0xC122734C, 0x2F1811F6, 0xA5114AFF, 0x93B93341, 0x6CEDD077, 0xCBABA04C, 0xB0873367, 0xDDBB2719, 0x8060C197, 0x17949273, 0x9EBA2656, 0xA6757910, 0xF832F645, 0x12836967, 0x2ED8B7ED, 0x501E8879, 0x920F899A, 0x6AEBB339, 0xB416EC59, 0xDCB94159, 0xD24DF92D, 0x2457DBC7, 0xBF6C9361, 0x6308ADE8, 0xB7E0A22C, 0xF8641498, 0x20367D0C, 0x994F70D2, 0x2E1B1007, 0x96EA933B, 0x65D11AD3, 0x78C0C565, 0xB2E327C9, 0xC9A3DB5, 0x2A5DA924, 0xFFBB3522, 0xA6187825, 0x14F47769, 0x3E3E549A, 0xDCBCE40A, 0x27323317, 0xECC35120, 0xAEEB915, 0xC47F38FD, 0x6A9DEF00, 0x2D7AC41D, 0x9E9EBA3C, 0xB5347E7, 0xE9F682DF, 0x936934BD, 0x715BC0D4, 0x7709F812, 0x44E0BDF7, 0x3BD64D43, 0xCCD014E8, 0x54F45FBE, 0x2E2A4FEE, 0x71FD352A, 0x527F1386, 0x81683594, 0x5370C3FF, 0xE19A6E62, 0x39978561, 0x4AAB76D6, 0x3A41D681, 0xB5EE9ED7, 0x465EA91C, 0xEF7222B1, 0x8098DF3C, 0x975E545D, 0xDE5D873A, 0x2FD8CF10, 0xFF806337, 0xDF0246B0, 0x12ABF503, 0xB185DA9D, 0xE1AD50CB, 0x746198FC, 0x86727129, 0x48A7C5FE, 0xFCA33C4C, 0xC1B4D550, 0xF2FC1DFE, 0x13DE163B, 0x7E9624D7, 0x45C84418, 0xCF8DEA12, 0xB7F566C7, 0x46CFC115, 0x81E6CDC4, 0xD8BCFF70, 0xB41D8E6C, 0x9F7ABF3F, 0x3FCD96B, 0x40203895, 0xDBE711C6, 0xEDDB977A, 0x4F0F98A1, 0xE4060DC6, 0x34AC5734, 0x884CDEB5, 0x860425FA, 0xB9D26FF9, 0x89E2B6CF, 0x94351640, 0xC2F3797, 0xC944A9CC, 0xC5570B76, 0x15CF86D7, 0x5455267A, 0xDF827B3, 0x89C4D05D, 0xD3C161C5, 0x71F3A803, 0x275ECEA5, 0xCE1A255, 0x4B055080, 0x6E115E15, 0x1D64B0AA, 0xF1A92FD5, 0x9A1B3206, 0x37FC92CA, 0x62EE5AFA, 0xD18E769A, 0xF5CCF050, 0xEC82A614]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 32];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u32, 128); }
    
    print!("RSA_4096_u32: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF, 0x11111111111111111111111111111111, 0x22222222222222222222222222222222, 0x33333333333333333333333333333333, 0x44444444444444444444444444444444, 0x55555555555555555555555555555555, 0x66666666666666666666666666666666, 0x77777777777777777777777777777777, 0x88888888888888888888888888888888, 0x99999999999999999999999999999999, 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0xCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0xDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x0]);
    assert_eq!(rr, message);

    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u16: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u16; 64];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr(), 64); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_1024_u16: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0xD910, 0x5876, 0x1BA1, 0x9DF, 0xBEA9, 0x3D2A, 0xF9F1, 0xF9EA, 0xB4, 0xB4DD, 0x8994, 0x423, 0x74F8, 0x9465, 0xD2B7, 0xFB00, 0xE91D, 0xD3F9, 0x8E8F, 0x26D7, 0xB93C, 0x4D1, 0xC3CC, 0x7666, 0xA323, 0x492E, 0xA926, 0xC7EF, 0xD5CE, 0x95E9, 0x4732, 0xF32C, 0x103D, 0x748, 0x76A7, 0x2985, 0xBA6D, 0x4342, 0xD1A9, 0xFF59, 0x85DD, 0x9D15, 0x236F, 0xFA9B, 0x82E2, 0x7B79, 0x425C, 0x2B80, 0xA9DA, 0x1112, 0x8BC7, 0xA5EA, 0x7FBB, 0x41AF, 0x6333, 0x557B, 0xD78B, 0xFA12, 0x451C, 0x34B8, 0xF90A, 0x867D, 0xE61E, 0xB412]);

    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 8];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u16, 64); }
    
    print!("RSA_1024_u16: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u16: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u16; 128];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr(), 128); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_2048_u16: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x3873, 0x4728, 0xEDD8, 0x2CDB, 0x2441, 0x9392, 0x2C0F, 0x8E8A, 0x3CDF, 0xD248, 0x502C, 0x712, 0xB948, 0x975F, 0x7C70, 0xA547, 0xE7A4, 0x5151, 0xA1E1, 0x7B83, 0x157F, 0x63B4, 0x25EB, 0x9CB2, 0x588C, 0xF613, 0x6E71, 0xA28B, 0x7440, 0xD16F, 0x799C, 0xE80, 0x9F2E, 0xE562, 0xDD76, 0x59A4, 0xAF66, 0x5CC7, 0x5732, 0x17FD, 0x1987, 0x90E3, 0xBDDD, 0x23B5, 0x9668, 0x2A32, 0x607E, 0x91DA, 0x9B3B, 0xE2EB, 0x378F, 0x1DFC, 0xB8F6, 0x56EF, 0xD998, 0xA387, 0x9286, 0x7C81, 0x1AC3, 0x7FFD, 0x143F, 0x81E4, 0x2533, 0x72B1, 0x1DC6, 0x6C87, 0xDDA8, 0x3317, 0x5497, 0x415F, 0x9FFD, 0x2FB5, 0xE361, 0xF720, 0xB9B5, 0x515D, 0xC7D1, 0x8EB0, 0xA272, 0x2747, 0xB141, 0x661A, 0x561A, 0xFA4, 0xAD98, 0x8E9D, 0x9098, 0xC3DC, 0x2E30, 0x6FC, 0x919D, 0x6CCC, 0xC6F4, 0x44BC, 0x5FD1, 0xCA83, 0xA677, 0xD660, 0x5E1F, 0xFC23, 0xCE65, 0x2D54, 0x846F, 0x16AD, 0xDBC3, 0xB8FB, 0xF02E, 0x5EB, 0x29D6, 0x63EE, 0x2F3B, 0x4926, 0x8F26, 0x96CC, 0x700C, 0x4BCD, 0x37A, 0x3362, 0xB057, 0xD909, 0x2D0D, 0x2D93, 0xBCCB, 0x1394, 0xD5DF, 0xE545, 0xD012, 0x6A33]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 16];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u16, 128); }
    
    print!("RSA_2048_u16: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u16: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u16; 256];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr(), 256); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_4096_u16: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");

    assert_eq!(cipher, [0xED19, 0x9CB6, 0x55D8, 0xD4FB, 0x734C, 0xC122, 0x11F6, 0x2F18, 0x4AFF, 0xA511, 0x3341, 0x93B9, 0xD077, 0x6CED, 0xA04C, 0xCBAB, 0x3367, 0xB087, 0x2719, 0xDDBB, 0xC197, 0x8060, 0x9273, 0x1794, 0x2656, 0x9EBA, 0x7910, 0xA675, 0xF645, 0xF832, 0x6967, 0x1283, 0xB7ED, 0x2ED8, 0x8879, 0x501E, 0x899A, 0x920F, 0xB339, 0x6AEB, 0xEC59, 0xB416, 0x4159, 0xDCB9, 0xF92D, 0xD24D, 0xDBC7, 0x2457, 0x9361, 0xBF6C, 0xADE8, 0x6308, 0xA22C, 0xB7E0, 0x1498, 0xF864, 0x7D0C, 0x2036, 0x70D2, 0x994F, 0x1007, 0x2E1B, 0x933B, 0x96EA, 0x1AD3, 0x65D1, 0xC565, 0x78C0, 0x27C9, 0xB2E3, 0x3DB5, 0xC9A, 0xA924, 0x2A5D, 0x3522, 0xFFBB, 0x7825, 0xA618, 0x7769, 0x14F4, 0x549A, 0x3E3E, 0xE40A, 0xDCBC, 0x3317, 0x2732, 0x5120, 0xECC3, 0xB915, 0xAEE, 0x38FD, 0xC47F, 0xEF00, 0x6A9D, 0xC41D, 0x2D7A, 0xBA3C, 0x9E9E, 0x47E7, 0xB53, 0x82DF, 0xE9F6, 0x34BD, 0x9369, 0xC0D4, 0x715B, 0xF812, 0x7709, 0xBDF7, 0x44E0, 0x4D43, 0x3BD6, 0x14E8, 0xCCD0, 0x5FBE, 0x54F4, 0x4FEE, 0x2E2A, 0x352A, 0x71FD, 0x1386, 0x527F, 0x3594, 0x8168, 0xC3FF, 0x5370, 0x6E62, 0xE19A, 0x8561, 0x3997, 0x76D6, 0x4AAB, 0xD681, 0x3A41, 0x9ED7, 0xB5EE, 0xA91C, 0x465E, 0x22B1, 0xEF72, 0xDF3C, 0x8098, 0x545D, 0x975E, 0x873A, 0xDE5D, 0xCF10, 0x2FD8, 0x6337, 0xFF80, 0x46B0, 0xDF02, 0xF503, 0x12AB, 0xDA9D, 0xB185, 0x50CB, 0xE1AD, 0x98FC, 0x7461, 0x7129, 0x8672, 0xC5FE, 0x48A7, 0x3C4C, 0xFCA3, 0xD550, 0xC1B4, 0x1DFE, 0xF2FC, 0x163B, 0x13DE, 0x24D7, 0x7E96, 0x4418, 0x45C8, 0xEA12, 0xCF8D, 0x66C7, 0xB7F5, 0xC115, 0x46CF, 0xCDC4, 0x81E6, 0xFF70, 0xD8BC, 0x8E6C, 0xB41D, 0xBF3F, 0x9F7A, 0xD96B, 0x3FC, 0x3895, 0x4020, 0x11C6, 0xDBE7, 0x977A, 0xEDDB, 0x98A1, 0x4F0F, 0xDC6, 0xE406, 0x5734, 0x34AC, 0xDEB5, 0x884C, 0x25FA, 0x8604, 0x6FF9, 0xB9D2, 0xB6CF, 0x89E2, 0x1640, 0x9435, 0x3797, 0xC2F, 0xA9CC, 0xC944, 0xB76, 0xC557, 0x86D7, 0x15CF, 0x267A, 0x5455, 0x27B3, 0xDF8, 0xD05D, 0x89C4, 0x61C5, 0xD3C1, 0xA803, 0x71F3, 0xCEA5, 0x275E, 0xA255, 0xCE1, 0x5080, 0x4B05, 0x5E15, 0x6E11, 0xB0AA, 0x1D64, 0x2FD5, 0xF1A9, 0x3206, 0x9A1B, 0x92CA, 0x37FC, 0x5AFA, 0x62EE, 0x769A, 0xD18E, 0xF050, 0xF5CC, 0xA614, 0xEC82]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 32];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u16, 256); }
    
    print!("RSA_4096_u16: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF, 0x11111111111111111111111111111111, 0x22222222222222222222222222222222, 0x33333333333333333333333333333333, 0x44444444444444444444444444444444, 0x55555555555555555555555555555555, 0x66666666666666666666666666666666, 0x77777777777777777777777777777777, 0x88888888888888888888888888888888, 0x99999999999999999999999999999999, 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0xCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0xDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x0]);
    assert_eq!(rr, message);

    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let public_key = U1024::from(5_u8);
    let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    let modulus = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];

    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u8: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u8; 128];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, m.as_mut_ptr(), 128); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_1024_u8: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x10, 0xD9, 0x76, 0x58, 0xA1, 0x1B, 0xDF, 0x9, 0xA9, 0xBE, 0x2A, 0x3D, 0xF1, 0xF9, 0xEA, 0xF9, 0xB4, 0x0, 0xDD, 0xB4, 0x94, 0x89, 0x23, 0x4, 0xF8, 0x74, 0x65, 0x94, 0xB7, 0xD2, 0x0, 0xFB, 0x1D, 0xE9, 0xF9, 0xD3, 0x8F, 0x8E, 0xD7, 0x26, 0x3C, 0xB9, 0xD1, 0x4, 0xCC, 0xC3, 0x66, 0x76, 0x23, 0xA3, 0x2E, 0x49, 0x26, 0xA9, 0xEF, 0xC7, 0xCE, 0xD5, 0xE9, 0x95, 0x32, 0x47, 0x2C, 0xF3, 0x3D, 0x10, 0x48, 0x7, 0xA7, 0x76, 0x85, 0x29, 0x6D, 0xBA, 0x42, 0x43, 0xA9, 0xD1, 0x59, 0xFF, 0xDD, 0x85, 0x15, 0x9D, 0x6F, 0x23, 0x9B, 0xFA, 0xE2, 0x82, 0x79, 0x7B, 0x5C, 0x42, 0x80, 0x2B, 0xDA, 0xA9, 0x12, 0x11, 0xC7, 0x8B, 0xEA, 0xA5, 0xBB, 0x7F, 0xAF, 0x41, 0x33, 0x63, 0x7B, 0x55, 0x8B, 0xD7, 0x12, 0xFA, 0x1C, 0x45, 0xB8, 0x34, 0xA, 0xF9, 0x7D, 0x86, 0x1E, 0xE6, 0x12, 0xB4]);

    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 8];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u8, 128); }
    
    print!("RSA_1024_u8: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let public_key = U2048::from(7_u8);
    let private_key = U2048::from_str_radix("4935A7A0CA0F94101C34BC4A3180F18FE756C4CAFAFDED502AF6B7EC89E42D89C0163CFD43CC58F90F9CEA3045B565957732102340ACF51695E3F635FBD3AAEE83F71EB37103D234B108380932372E677200AD37074BBCEEC60FBEE4AB73C8F7030F712C8A70B43A05BD10700A0C50579CE8A8AEEF96D1D8BF7EDA7CF946F64A04F4C739591D41E2D0CD9E65FA6CED2AAEAE2BB3E9CE38BD0BA06DFF1847965DBE264447EE0C1AE7452638014AA2959B1FEAE409987C65321651C896732A62F0581F87A94DC41D85C3936C369A803DB0D6BD99B346C01A74295C24260E0055BFA97013967F9D6A868E57A67EBF17A6C9D7D5FB50BEFA4A4AD93BC558B8A78297", 16).unwrap();
    let modulus = U2048::from_str_radix("aad28721d779aed041d06202738233a51bca75d99efb29bb0eea57d297146a416ade8e4ef3877a45246e2270a2a7425cc0ca25a796e8e68a08693e7df6433981de95f24d5d08ea7af26882c01fd616f15f56e980665b638278cf6815900e2a405c795d67edb1a48762b9265ac21cbb7718c989982f0a944f1427fdce45a59403b080b68d260dafd1fb64b374aaac80bd1f595bd0ff175ca6108fdd2f83905774f66828ba84fab66412430666d46d1b94bb0da1c9d9c9062497677ea5b6b751f1e51651d86b6d377c3b44767134496a8fe2d7157e5f17281ad156d1a05eea359bc847d2d835474e1d4dc517902ffcfb036f7444bf296ad86ab0795f058a6a7a4f", 16).unwrap();
    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF];

    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u8: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u8; 256];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, m.as_mut_ptr(), 256); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_2048_u8: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");
    assert_eq!(cipher, [0x73, 0x38, 0x28, 0x47, 0xD8, 0xED, 0xDB, 0x2C, 0x41, 0x24, 0x92, 0x93, 0xF, 0x2C, 0x8A, 0x8E, 0xDF, 0x3C, 0x48, 0xD2, 0x2C, 0x50, 0x12, 0x7, 0x48, 0xB9, 0x5F, 0x97, 0x70, 0x7C, 0x47, 0xA5, 0xA4, 0xE7, 0x51, 0x51, 0xE1, 0xA1, 0x83, 0x7B, 0x7F, 0x15, 0xB4, 0x63, 0xEB, 0x25, 0xB2, 0x9C, 0x8C, 0x58, 0x13, 0xF6, 0x71, 0x6E, 0x8B, 0xA2, 0x40, 0x74, 0x6F, 0xD1, 0x9C, 0x79, 0x80, 0xE, 0x2E, 0x9F, 0x62, 0xE5, 0x76, 0xDD, 0xA4, 0x59, 0x66, 0xAF, 0xC7, 0x5C, 0x32, 0x57, 0xFD, 0x17, 0x87, 0x19, 0xE3, 0x90, 0xDD, 0xBD, 0xB5, 0x23, 0x68, 0x96, 0x32, 0x2A, 0x7E, 0x60, 0xDA, 0x91, 0x3B, 0x9B, 0xEB, 0xE2, 0x8F, 0x37, 0xFC, 0x1D, 0xF6, 0xB8, 0xEF, 0x56, 0x98, 0xD9, 0x87, 0xA3, 0x86, 0x92, 0x81, 0x7C, 0xC3, 0x1A, 0xFD, 0x7F, 0x3F, 0x14, 0xE4, 0x81, 0x33, 0x25, 0xB1, 0x72, 0xC6, 0x1D, 0x87, 0x6C, 0xA8, 0xDD, 0x17, 0x33, 0x97, 0x54, 0x5F, 0x41, 0xFD, 0x9F, 0xB5, 0x2F, 0x61, 0xE3, 0x20, 0xF7, 0xB5, 0xB9, 0x5D, 0x51, 0xD1, 0xC7, 0xB0, 0x8E, 0x72, 0xA2, 0x47, 0x27, 0x41, 0xB1, 0x1A, 0x66, 0x1A, 0x56, 0xA4, 0xF, 0x98, 0xAD, 0x9D, 0x8E, 0x98, 0x90, 0xDC, 0xC3, 0x30, 0x2E, 0xFC, 0x6, 0x9D, 0x91, 0xCC, 0x6C, 0xF4, 0xC6, 0xBC, 0x44, 0xD1, 0x5F, 0x83, 0xCA, 0x77, 0xA6, 0x60, 0xD6, 0x1F, 0x5E, 0x23, 0xFC, 0x65, 0xCE, 0x54, 0x2D, 0x6F, 0x84, 0xAD, 0x16, 0xC3, 0xDB, 0xFB, 0xB8, 0x2E, 0xF0, 0xEB, 0x5, 0xD6, 0x29, 0xEE, 0x63, 0x3B, 0x2F, 0x26, 0x49, 0x26, 0x8F, 0xCC, 0x96, 0xC, 0x70, 0xCD, 0x4B, 0x7A, 0x3, 0x62, 0x33, 0x57, 0xB0, 0x9, 0xD9, 0xD, 0x2D, 0x93, 0x2D, 0xCB, 0xBC, 0x94, 0x13, 0xDF, 0xD5, 0x45, 0xE5, 0x12, 0xD0, 0x33, 0x6A]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 16];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u8, 256); }
    
    print!("RSA_2048_u8: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF]);
    assert_eq!(rr, message);

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("65D454FB951A5CEFCA5203530570263DC58CA1F8C489801223754B92F3AFAFCF390C35A6C4F01FBFB6F70563146F1A5610816C34781F831871D946665E7752B92E522A1556DFD885332EC38E0A65A3D934C40C1B123A8181EFC7BE48657EC3437FED43EE8197E2E2F61F639DFEEBA157ED5F33B748A5ADE0881B8A377DFB4FD889732DE0F5DE07454E1845E64CD28321CF7D1B7EBACAE56F5B2EE12232927312A7D4113EACF69903245F86587EFDA359E19CAF07884D3DDD82482B1A8ACBA989933014EED53FAE4DB84A97F2CCD8CBC0C769E909CE918785EB8CA6C13232F0D88F8A0F2A124815BA1B6F5991D258B3262F012FE931BB50E90D1EB9FE3793CD8A18182768BCC1926552BDA2F452FFE5A25DADBF36F83D01FB072B463841A939F0EFAFD7E6564E7F71705CDCFB1281AF424D4EB6282F1F33ADD58DD862B3DE496D6F379BAD1E068893C3A2FFC75DDB1DC18042569AFA0F9DE44A582534BE73DA29D40E624A82A19F6D48609609600FF34BCA8E4D69F18101A3F0BAE655D5F389D568BB91E54465E5AA0FB52CA6E372A21BCDEB45EA16FC36FEFDBC9FF3F295B53C25458C75EE869755A4E4557BD639DE5D80AA9FDE709A53F4D6D135FD8611D0EED5CBB9430C13292C74DF5380BCDE17A4C3BF0E692E505683560CBA89C931934C76B4C499A5092C586EE014732761CEB7C9D67BF43472D89644D2250DAA0E82CD", 16).unwrap();
    let modulus = U4096::from_str_radix("7f496a3a7a60f42bbce68427c6cc2fcd36efca76f5abe016ac529e77b09b9bc3074f4310762c27afa4b4c6bbd98ae0eb94a1c741962763de8e4f97fff615276779e6b49aac97cea67ffa74718cff0ccf81f50f21d6c921e26bb9adda7ede74145fe894ea21fddb9bb3a73c857ea689ade8b700a51acf1958aa226cc55d7a23ceabcff95933558916a19e575fe00723ea435c625e697d9ecb31fa996abf370fd751c9158e58343f43ed7767ee9ebd0c305a03dac96a608d54e2da35e12d7e93ebf7fc1a2a8a8f99e1265d3def800efeb0f944634c4235e967666fd0717ebfad0eb36c92f496da1b28a24b2ff646eedfefbac17be37e2a25235066687dc578c0ee1a0790c4437f848078eefd7082f84fc343c1accf9cc24f5e8e0eb021f4dd25fd308c91a79c416639c63e8ad87d452a2d25b4d37356a8d8e70b82df6226fbda1fb7d5a6346d18a9ec79401d6bb3776fa2115eb1c6bc83213eef559f10d0cd21a6d159f3a8910fc13d3a2c031eb3394e7a7928df1d854122b0dc032cb3272adc131ed8bd6e82af43294db9b0640189ebf6da9922af531638f7f71282c240518559e3c839767d78b5b5031f1a88b13028a55587f15a047cd2f565bad97a9a50afab1699397dcc92f10bc0140d2a796686253eabf412ea85e70c0c1d47a22b803fcf94af4ad3dda0dca583ff830b57b2cf86e1c2c0dd15b2a26372f3e43a1300226d", 16).unwrap();
    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000];

    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u8: Message = [");
    for m in message
        { print!("{:#X}, ", m); }
    println!("]");

    let mut m = [0u8; 512];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, m.as_mut_ptr(), 512); }
    let cipher = rsa.encrypt_unit(&m);
    print!("RSA_4096_u8: Cipher = [");
    for c in cipher
        { print!("{:#X}, ", c); }
    println!("]");

    assert_eq!(cipher, [0x19, 0xED, 0xB6, 0x9C, 0xD8, 0x55, 0xFB, 0xD4, 0x4C, 0x73, 0x22, 0xC1, 0xF6, 0x11, 0x18, 0x2F, 0xFF, 0x4A, 0x11, 0xA5, 0x41, 0x33, 0xB9, 0x93, 0x77, 0xD0, 0xED, 0x6C, 0x4C, 0xA0, 0xAB, 0xCB, 0x67, 0x33, 0x87, 0xB0, 0x19, 0x27, 0xBB, 0xDD, 0x97, 0xC1, 0x60, 0x80, 0x73, 0x92, 0x94, 0x17, 0x56, 0x26, 0xBA, 0x9E, 0x10, 0x79, 0x75, 0xA6, 0x45, 0xF6, 0x32, 0xF8, 0x67, 0x69, 0x83, 0x12, 0xED, 0xB7, 0xD8, 0x2E, 0x79, 0x88, 0x1E, 0x50, 0x9A, 0x89, 0xF, 0x92, 0x39, 0xB3, 0xEB, 0x6A, 0x59, 0xEC, 0x16, 0xB4, 0x59, 0x41, 0xB9, 0xDC, 0x2D, 0xF9, 0x4D, 0xD2, 0xC7, 0xDB, 0x57, 0x24, 0x61, 0x93, 0x6C, 0xBF, 0xE8, 0xAD, 0x8, 0x63, 0x2C, 0xA2, 0xE0, 0xB7, 0x98, 0x14, 0x64, 0xF8, 0xC, 0x7D, 0x36, 0x20, 0xD2, 0x70, 0x4F, 0x99, 0x7, 0x10, 0x1B, 0x2E, 0x3B, 0x93, 0xEA, 0x96, 0xD3, 0x1A, 0xD1, 0x65, 0x65, 0xC5, 0xC0, 0x78, 0xC9, 0x27, 0xE3, 0xB2, 0xB5, 0x3D, 0x9A, 0xC, 0x24, 0xA9, 0x5D, 0x2A, 0x22, 0x35, 0xBB, 0xFF, 0x25, 0x78, 0x18, 0xA6, 0x69, 0x77, 0xF4, 0x14, 0x9A, 0x54, 0x3E, 0x3E, 0xA, 0xE4, 0xBC, 0xDC, 0x17, 0x33, 0x32, 0x27, 0x20, 0x51, 0xC3, 0xEC, 0x15, 0xB9, 0xEE, 0xA, 0xFD, 0x38, 0x7F, 0xC4, 0x0, 0xEF, 0x9D, 0x6A, 0x1D, 0xC4, 0x7A, 0x2D, 0x3C, 0xBA, 0x9E, 0x9E, 0xE7, 0x47, 0x53, 0xB, 0xDF, 0x82, 0xF6, 0xE9, 0xBD, 0x34, 0x69, 0x93, 0xD4, 0xC0, 0x5B, 0x71, 0x12, 0xF8, 0x9, 0x77, 0xF7, 0xBD, 0xE0, 0x44, 0x43, 0x4D, 0xD6, 0x3B, 0xE8, 0x14, 0xD0, 0xCC, 0xBE, 0x5F, 0xF4, 0x54, 0xEE, 0x4F, 0x2A, 0x2E, 0x2A, 0x35, 0xFD, 0x71, 0x86, 0x13, 0x7F, 0x52, 0x94, 0x35, 0x68, 0x81, 0xFF, 0xC3, 0x70, 0x53, 0x62, 0x6E, 0x9A, 0xE1, 0x61, 0x85, 0x97, 0x39, 0xD6, 0x76, 0xAB, 0x4A, 0x81, 0xD6, 0x41, 0x3A, 0xD7, 0x9E, 0xEE, 0xB5, 0x1C, 0xA9, 0x5E, 0x46, 0xB1, 0x22, 0x72, 0xEF, 0x3C, 0xDF, 0x98, 0x80, 0x5D, 0x54, 0x5E, 0x97, 0x3A, 0x87, 0x5D, 0xDE, 0x10, 0xCF, 0xD8, 0x2F, 0x37, 0x63, 0x80, 0xFF, 0xB0, 0x46, 0x2, 0xDF, 0x3, 0xF5, 0xAB, 0x12, 0x9D, 0xDA, 0x85, 0xB1, 0xCB, 0x50, 0xAD, 0xE1, 0xFC, 0x98, 0x61, 0x74, 0x29, 0x71, 0x72, 0x86, 0xFE, 0xC5, 0xA7, 0x48, 0x4C, 0x3C, 0xA3, 0xFC, 0x50, 0xD5, 0xB4, 0xC1, 0xFE, 0x1D, 0xFC, 0xF2, 0x3B, 0x16, 0xDE, 0x13, 0xD7, 0x24, 0x96, 0x7E, 0x18, 0x44, 0xC8, 0x45, 0x12, 0xEA, 0x8D, 0xCF, 0xC7, 0x66, 0xF5, 0xB7, 0x15, 0xC1, 0xCF, 0x46, 0xC4, 0xCD, 0xE6, 0x81, 0x70, 0xFF, 0xBC, 0xD8, 0x6C, 0x8E, 0x1D, 0xB4, 0x3F, 0xBF, 0x7A, 0x9F, 0x6B, 0xD9, 0xFC, 0x3, 0x95, 0x38, 0x20, 0x40, 0xC6, 0x11, 0xE7, 0xDB, 0x7A, 0x97, 0xDB, 0xED, 0xA1, 0x98, 0xF, 0x4F, 0xC6, 0xD, 0x6, 0xE4, 0x34, 0x57, 0xAC, 0x34, 0xB5, 0xDE, 0x4C, 0x88, 0xFA, 0x25, 0x4, 0x86, 0xF9, 0x6F, 0xD2, 0xB9, 0xCF, 0xB6, 0xE2, 0x89, 0x40, 0x16, 0x35, 0x94, 0x97, 0x37, 0x2F, 0xC, 0xCC, 0xA9, 0x44, 0xC9, 0x76, 0xB, 0x57, 0xC5, 0xD7, 0x86, 0xCF, 0x15, 0x7A, 0x26, 0x55, 0x54, 0xB3, 0x27, 0xF8, 0xD, 0x5D, 0xD0, 0xC4, 0x89, 0xC5, 0x61, 0xC1, 0xD3, 0x3, 0xA8, 0xF3, 0x71, 0xA5, 0xCE, 0x5E, 0x27, 0x55, 0xA2, 0xE1, 0xC, 0x80, 0x50, 0x5, 0x4B, 0x15, 0x5E, 0x11, 0x6E, 0xAA, 0xB0, 0x64, 0x1D, 0xD5, 0x2F, 0xA9, 0xF1, 0x6, 0x32, 0x1B, 0x9A, 0xCA, 0x92, 0xFC, 0x37, 0xFA, 0x5A, 0xEE, 0x62, 0x9A, 0x76, 0x8E, 0xD1, 0x50, 0xF0, 0xCC, 0xF5, 0x14, 0xA6, 0x82, 0xEC]);
    let recovered = rsa.decrypt_unit(&cipher);
    let mut rr = [0u128; 32];
    unsafe { copy_nonoverlapping(recovered.as_ptr(), rr.as_mut_ptr() as *mut u8, 512); }
    
    print!("RSA_4096_u8: Recovered = [");
    for r in rr
        { print!("{:#X}, ", r); }
    println!("]");
    assert_eq!(rr, [0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6079889706A5B4C3D2E1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF, 0xFEDCBA98765432100123456789ABCDEF, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x879605A4B3C2D1E00F1E2D3C4B5A6978, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x111122223333444499990000AAAABBBB, 0x5555666677778888CCCCDDDDEEEEFFFF, 0x11111111111111111111111111111111, 0x22222222222222222222222222222222, 0x33333333333333333333333333333333, 0x44444444444444444444444444444444, 0x55555555555555555555555555555555, 0x66666666666666666666666666666666, 0x77777777777777777777777777777777, 0x88888888888888888888888888888888, 0x99999999999999999999999999999999, 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0xCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0xDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x0]);
    assert_eq!(rr, message);
    println!("-------------------------------");
}

fn rsa_decrypt_array_unit()
{
    println!("rsa_decrypt_array_unit");
    use std::ptr::copy_nonoverlapping;
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 32]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 32 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_1024: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xE181EC1C, 0x86C53D7E, 0xC1EE84B, 0xC64EE28F, 0xBCB1272C, 0xDCCF7F8D, 0x5CCEE57E, 0xDEE7F806, 0x260E66E3, 0xEAF66FB4, 0x84C29D28, 0xF3331A75, 0x6A98F58E, 0xAF51309B, 0x988A192A, 0x36D16E3D, 0x8F7D9425, 0xD125283B, 0x64A9AC13, 0xBA293535, 0x48A937F6, 0x347A3932, 0x678E3AA9, 0x596C50E5, 0x372F32F3, 0x92CADF0A, 0x53A41E0, 0x527CDD75, 0x637C4679, 0x16575D1C, 0x1E452703, 0xC2A8AAFA],
                        [0x6E971C27, 0x9D289F9E, 0x5890CBE1, 0xDCB604D0, 0x3DFF02E4, 0x8BB80EDF, 0x16025971, 0x47F83DFB, 0x15DB89ED, 0x37D3D81E, 0x1C784012, 0x96B5A8B2, 0x9B73CDEB, 0x19C3EB1C, 0x7380AF9B, 0xC7C05B11, 0x8778FFB1, 0xA4980451, 0x7817EF0E, 0xFBEC9BB0, 0x775226C3, 0x17DF9B7F, 0xBAD86876, 0xD482C2D8, 0x3AD54CCB, 0x63206A15, 0x5380238F, 0xCF893CDA, 0xD918FB53, 0x278CC36, 0x88966E4F, 0xED9F1A8A],
                        [0x732F2B30, 0x4FF2B312, 0x764BC5FE, 0x1A83AAC2, 0x6B97E97A, 0x8740A298, 0xD0F1CCD, 0x6BE5488D, 0x55BCFAB6, 0x44D8DD33, 0xF0BDF5E3, 0xA4F2A20F, 0x5ED64723, 0xDFFEA2C7, 0xBB55E1E, 0xBA258D21, 0x75A9D8B8, 0x41087FE1, 0xF73A3142, 0x97551901, 0xB4534FEC, 0x94EAAEF3, 0x34A07F91, 0x3A31B17E, 0x6C49CF97, 0xAB7A50A1, 0xEAFD8E4, 0x3BDED4CB, 0x1360C56A, 0xA8053DEA, 0xF4967191, 0x9735C7FF]]);

    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 8]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 32 * 3); }
    
    print!("RSA_1024: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                        [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                        [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(rrr, message);

    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 64]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 64 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_2048: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x58512746, 0xA88DE309, 0x16A1FCA0, 0x839DAD1, 0x63C593A1, 0x73655BFB, 0x7991B26, 0xBFEF2D64, 0x8602856F, 0x6C4BFFD5, 0x4B6969DA, 0xFAE91CB1, 0xCE340203, 0x487811E8, 0x55FF5A2B, 0x3A344493, 0x40933E17, 0xB6E7FC8E, 0xD1BB3CA2, 0xCBF46CC4, 0xA799D9F6, 0xA71195B4, 0xC904ED6E, 0xCEE3C80, 0xC50713C0, 0x7C396E59, 0xC32D0579, 0x16839F92, 0xB7AACD80, 0x32913210, 0x66AB3112, 0xF885BEAC, 0x75BC7B1A, 0x8E4B37AF, 0xDC8E5EFA, 0xD55E6E53, 0x55168D6A, 0xAC8C176F, 0x55F529D7, 0x1CF6778B, 0x6927C346, 0xB828CD00, 0xFF1F2E96, 0x8733A73C, 0xDCF3EC8B, 0x107A6AC1, 0x87E4317B, 0x55EA897A, 0x484B9F8C, 0x7D109B38, 0x72E3C18C, 0x7A52100A, 0x69574CD9, 0xD2232D3E, 0xEC16E847, 0xA0AAF25E, 0x7812251E, 0x52727569, 0x44211691, 0x1A7CC156, 0x458B9D97, 0xF70A2641, 0x8D7CE45B, 0x144B8CDE],
                        [0xAF8EA34A, 0x8451F77A, 0x1CC9E10B, 0x1A6F8D28, 0x1B1E535B, 0xD636F52F, 0x16B759DC, 0x46A8B, 0xA4DB4947, 0x694603D4, 0xD47357D1, 0xFAAA44CB, 0x18A7D50A, 0x281FF567, 0xE31648FD, 0xB996F502, 0xF6FFC57F, 0x1E1509C7, 0xA3CBC1C6, 0x430A5970, 0x87501C03, 0xBDDE7C4B, 0x6A104F83, 0x9AC886D1, 0xAFE5FCB3, 0xCAE6D365, 0x781B31B4, 0x15689F6A, 0xEC393431, 0xAE042223, 0x94281CB9, 0x3E7EECD, 0xED46EEB9, 0xB958CB54, 0xF3F0D946, 0x41F39B10, 0xDDAA89AF, 0xC2152C2E, 0x2CCB18C5, 0xB6A22DD0, 0x4FEA1DC7, 0xE4E9D89D, 0x4B40BCCA, 0xAA738CC7, 0x1FCF3CBD, 0x30A41CA8, 0x9C4BF168, 0xE78C0962, 0xA96F42DE, 0x25FE30C7, 0x44970C8, 0x4957120, 0xF4BEA26E, 0xC8880E2, 0xB6DBFB74, 0x5DDA4CE5, 0x2AFE2CD0, 0xA6860B6E, 0x72E227CA, 0xB91E53CA, 0x7DC37372, 0xEC31BAC1, 0x6DFA0EF, 0x204ABDC6],
                        [0x3EC97176, 0x3E662231, 0xD7142C09, 0x872DF0A7, 0x912E04C9, 0x9A2EB827, 0xE5ED5CDF, 0x4A6CBDE9, 0x8AD9C195, 0x81EC6854, 0xDD5F05DD, 0x3D9005D, 0x3DB693E4, 0x406BA4B0, 0xCC9A7BA5, 0xC7F81B10, 0x5B0FEEA7, 0x78195A94, 0x45CC9F4A, 0x5C3686F6, 0xB17EA923, 0xCEA9D40F, 0x6543C935, 0xEB44EB69, 0x9C406F87, 0x464C2710, 0x3DEE373C, 0xED63CB32, 0xF784D1B4, 0x8DE9C7BA, 0x3160BC6, 0x38A4E187, 0xD7C49411, 0xF8486078, 0x40030B00, 0x97047FAD, 0xB837A89F, 0xB00E34F5, 0x16279A66, 0xD68F4FED, 0x470BD897, 0x5ED4AED0, 0x2BB360C2, 0x8E1A882C, 0x74A73D94, 0xDC1257F4, 0x5B8DFBA2, 0x64E737E5, 0x395C8152, 0xC7822C68, 0x7C9AD0B4, 0x69134CCD, 0xBD2535D6, 0x4F72EDF6, 0x1101F3A4, 0x4AA60B4C, 0xBB03EC35, 0xF886D1E7, 0xC85FADF, 0xE11617FB, 0xAA005E06, 0xBAD3992F, 0x90BD84BB, 0x10AA50CD]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 16]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 64 * 3); }
    
    print!("RSA_2048: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(rrr, message);

    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 128]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 128 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_4096: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x297D441C, 0x2C5F6411, 0x753A7FEE, 0x50C6112C, 0x9765F75A, 0xC967B73, 0x464B45EB, 0x2E7B2CE2, 0xAD8D127, 0xBE324F21, 0x718D227D, 0xAA644C8B, 0x478530AD, 0xB060802D, 0x5E688409, 0x82F3C894, 0x9930DB71, 0xA2287A36, 0xE9E5766F, 0xC09E5C5C, 0xB4DEBA1E, 0xDD452F9E, 0x5D195295, 0xA366D241, 0x5C0E152E, 0xEDAB47C6, 0x6BDE070E, 0x27A5AA6F, 0x134009FB, 0x55B1038D, 0x19196446, 0xC4A9C264, 0xAEC6AB51, 0xA437DCD5, 0xF8EBC560, 0x9EADB1FC, 0xA446D423, 0x6C10C632, 0x20558326, 0xEF92D5BC, 0x124F75C2, 0x61316515, 0x9BA2A06D, 0xC40239A3, 0x826440E8, 0xFF04ED8B, 0x3BE3A38F, 0x8FFBEDA7, 0xBB154B25, 0xF37A69D3, 0x82A21E50, 0xE799865E, 0xB9FC1E91, 0x8DDA3FF8, 0xA92673B5, 0x2E6B7BDF, 0x2C6630AB, 0xF10FFFAF, 0xD1EC6253, 0xEE7B7D26, 0xE367BC27, 0x6233235A, 0x821954B5, 0x7516FFA2, 0x1BBF54E5, 0xB9A7901E, 0xC6672059, 0xE70F4930, 0x39A64754, 0xD4C12243, 0x49CDEBB8, 0x2E39E8A1, 0xCAF382AD, 0x94D3E2FC, 0xEC85450F, 0x31D7D3A7, 0x2FC47057, 0x95BB0C4, 0xF7C96DC1, 0x4F99EF5C, 0x4B14F7D7, 0xA77AD0D6, 0xA1D4CD71, 0xCF8D9BB4, 0x6F9EB898, 0x367BD1B, 0x6118438B, 0x558B0A29, 0xC7E346CA, 0x3721EBBF, 0xA44449C8, 0x1F26303F, 0x4205F7A3, 0x80224033, 0xCD0F2DFF, 0xABD140D0, 0x9AD41EDD, 0xD51B8A18, 0x3291E8D8, 0xC09A4F19, 0xA53D31ED, 0xA6BDBAB9, 0x5BC8D0EC, 0xBF50027C, 0xD8EFD4C7, 0x98A46C2E, 0xF1016DBF, 0x8960EBE0, 0x5783CE6E, 0x8C5F0A55, 0xC64D7A6B, 0x86B53F64, 0x85D2EF84, 0x22679606, 0x97A089B1, 0xCB8C74E1, 0xA3CCC592, 0xEFFE6EC3, 0x4F79D2D0, 0x389545E1, 0x84719E7C, 0xDC0004BA, 0x86CAF327, 0xBC113DF7, 0x875A5C24, 0xD93A4A0D, 0x7A8AD437, 0xDD5831F7],
                        [0xF8EA263C, 0xE551A4FE, 0xF20B52DF, 0xC992089E, 0x74176FCA, 0x75DC52BC, 0xD7EF3A78, 0x9D510616, 0x76A595DC, 0xF44E4A6D, 0x6FF6C7D6, 0x9D06E31C, 0x1447F881, 0x5B10CD8B, 0x8E44C9F5, 0xDCB6326C, 0xB79F18C, 0xD008BBB1, 0xA9DAE98A, 0x62ACFF3E, 0x2A7C3CB7, 0xF774DD3D, 0x3041C333, 0x154367CF, 0x1DE4A90A, 0xB7DE12F, 0x8DAD4359, 0x8EB7CE1A, 0xB11561CE, 0xA03E2ACF, 0x2E3CC59, 0x29A27365, 0x884BCF0C, 0xC186776C, 0xBDB3F787, 0xA76CDC7C, 0x32D0EC90, 0x341DB3CE, 0xC308A18E, 0xBBC4F265, 0x8EA878C3, 0xBBE9D930, 0xD4C2339, 0xCB2DCA03, 0xAAA4361D, 0xF9086339, 0x851158E0, 0xDC6187A5, 0xB85DC071, 0xB0DFC87F, 0xD762704B, 0x587630DF, 0xB0F3AFEE, 0x77702F19, 0xE72DEAB1, 0xCC6F76A4, 0xFBF91FE9, 0x682257BB, 0xE1803C31, 0x4AA72B7D, 0x319536E0, 0xD3B607D4, 0xB249A538, 0x166C0187, 0xF121272D, 0x9F1089AB, 0x826266CD, 0x209A5B74, 0x51B7F6E4, 0x698A875E, 0xC89964C2, 0xE0114D4C, 0xE32315E8, 0xFBFD50C3, 0x632EA904, 0xC92137E4, 0xCCEE3076, 0x48F7122C, 0xF50773CD, 0x79EFE6C6, 0xECFD11AC, 0x1BA83BBC, 0xC5F23FDA, 0xC4FE9C9D, 0xDC6D0467, 0xE8A94956, 0xF96D707D, 0x63DC6592, 0x9AE27FEC, 0x800811FC, 0x47306665, 0xDD82AC4E, 0x36931217, 0x8B813528, 0x88E4F5FC, 0x98896DD4, 0xB80D2269, 0x6C19905A, 0xDBA6B482, 0x258B79A2, 0x1A3C9AA5, 0xC1A514BD, 0x6C2845D2, 0x72D0CB25, 0x18C1F440, 0xE769717, 0xD2D4D517, 0x669BB63A, 0x85737599, 0x8761B910, 0xD5AA71B7, 0x7A1123D0, 0xBCD95883, 0xA02A5025, 0x72B71CA3, 0x60E991E2, 0xAD6601C7, 0x9176C80F, 0x2151FCA9, 0x70FCE4A9, 0xA16B991C, 0x483B834F, 0x7216959F, 0x2707FC2F, 0x771A46E0, 0xEF3E168, 0x3D3CEFCC, 0x8DCA257D],
                        [0xF42BE309, 0xDB4CE2DC, 0x42A27BDD, 0xC198B82B, 0x3E7A2747, 0x46FF7A3E, 0x9EA7255B, 0x178A5D5F, 0x89FFAAD, 0x4EC09632, 0xE81D248F, 0x6835F752, 0xF6EC565D, 0xC1DAFB0, 0x2D0DF0B5, 0xDF30C11F, 0x36B1ED61, 0xD303EBF5, 0x20D05AC8, 0x8D3EBEA7, 0x63D136F6, 0x15CC58A, 0xC4A65B45, 0x17127731, 0x94F6B581, 0x11BB2E0, 0xAEA46599, 0x3E2B0D84, 0x6D5CD0FC, 0xDAE2987D, 0xBD857C7B, 0xDEC69E3A, 0x55669B9A, 0x1D25D802, 0x356C3B31, 0x63D78F4E, 0x55BA82A4, 0x31ECD988, 0xD0FAC275, 0xBCF6D009, 0x4D3BBE36, 0x8394AD0, 0x7849CBF8, 0x73F0F16B, 0x2D5ED807, 0xDE3BE531, 0x23C06EC6, 0x62A5A61, 0x97AD95D3, 0xB85AB016, 0x32B7326D, 0x3E857E82, 0xC2105D2E, 0xFE9085C1, 0xEF4AF832, 0xC52CB9A4, 0xF302F044, 0xA7B30970, 0xAB41472D, 0xBE4C22A5, 0xCFB4ACA8, 0xD63C7428, 0x5195A499, 0xCA9FFB1D, 0x7604D98C, 0x47A7DDD, 0xE7D11482, 0xF4B0C737, 0x85040FBC, 0xE39A51F7, 0xC51A71DD, 0x880981C0, 0xBF4CAF2F, 0x443873F4, 0x90DAFC99, 0x4823B425, 0x85C04D13, 0x5A016214, 0xB033E79D, 0x3A175AF4, 0xAB6FBAD1, 0xDE0BE592, 0x713C68FE, 0xE01ACB15, 0xE2410568, 0x4B77433D, 0xECF20668, 0xDB21D751, 0xE80C456D, 0x2D9D62C6, 0xBC29F1CD, 0x1E137E49, 0xECC20CD9, 0xAC1D53D2, 0x7C589B17, 0x48E555B7, 0x88418FB0, 0x5D5EC0B1, 0x16EA1D9D, 0x87EEC267, 0xB83A7E17, 0xB9E2E216, 0x27C1B230, 0x147902F6, 0x525D2FB5, 0x707C4CDE, 0xBCF186FA, 0x85B9DAAC, 0x5E9AFE8A, 0x2DBB8CF9, 0x6D61C17C, 0xF1F2AE4C, 0x1ECCC2BF, 0xFB5177CD, 0x94820CC2, 0x98004AFB, 0xEBE22490, 0xD2ECD1A0, 0xE6B0697, 0x9E2C9D4, 0xB62101F, 0xDD9D5027, 0xC4D07CC1, 0xC4714130, 0x87F511B, 0x107E5ABF, 0xE1E238E6, 0xD2114B52]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 32]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 128 * 3); }
    
    print!("RSA_4096: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(rrr, message);

    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Generic;

    let public_key = BigUInt::<u64, 4>::from(5_u8);
    let private_key = BigUInt::<u64, 4>::from_str_radix("92D109331B85A642EC6AE3F68A4E7272426A4BD0A0C16EF9B62F9269ACF9E8CD", 16).unwrap();
    let modulus = BigUInt::<u64, 4>::from_str_radix("b7854b7fe2670fd3a7859cf42ce20f1087aad03f863107e2b7bac58e09483f3d", 16).unwrap();
    let rsa = RSA_Generic::<4, u64, 5>::new_with_keys(public_key, private_key, modulus);
    let message = [[0x_FEDCBA9876543210_u64, 0x_1122334455667788, 0x_9900AABBCCDDEEFF, 0x_FEDCBA0987654321],
                    [0x_10FEDCBA98765432_u64, 0x_8811223344556677, 0x_CCDDEEFF9900AABB, 0x_9876FEDCBA054321],
                    [0x_3210FEDCBA987654_u64, 0x_7788112233445566, 0x_9900EEFFAABBCCDD, 0x_FEDC54321BA09876]];

    println!("RSA_Generic<4, u64, 5>: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_Generic<4, u64, 5>: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_Generic<4, u64, 5>: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let cipher = rsa.encrypt_array_unit(&message);
    print!("RSA_Generic<4, u64, 5>: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xEB94831A3D67B97E, 0x14A5FE7C9A1F8E92, 0x9EAA729DE8B91D5B, 0x3065877D5C71FB46],
                        [0xBDD4B28968F07A29, 0x4FB31BF06504F164, 0x931AD673E14E379B, 0xC3021414384B0C21],
                        [0x299756779EBC6378, 0x5159A86D671F491C, 0xD7FF0A7562B5EAD9, 0xEEB151D0770DC2BC]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    print!("RSA_Generic<4, u64, 5>: Recovered = [");
    for rr in recovered
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(recovered, [[0x_FEDCBA9876543210_u64, 0x_1122334455667788, 0x_9900AABBCCDDEEFF, 0x_FEDCBA0987654321],
                    [0x_10FEDCBA98765432_u64, 0x_8811223344556677, 0x_CCDDEEFF9900AABB, 0x_9876FEDCBA054321],
                    [0x_3210FEDCBA987654_u64, 0x_7788112233445566, 0x_9900EEFFAABBCCDD, 0x_FEDC54321BA09876]]);
    assert_eq!(recovered, message);

    // Example for RSA_1024_u128
    use cryptocol::asymmetric::RSA_1024_u128;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u128: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let cipher = rsa.encrypt_array_unit(&message);
    print!("RSA_1024_u128: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xC64EE28F0C1EE84B86C53D7EE181EC1C, 0xDEE7F8065CCEE57EDCCF7F8DBCB1272C, 0xF3331A7584C29D28EAF66FB4260E66E3, 0x36D16E3D988A192AAF51309B6A98F58E, 0xBA29353564A9AC13D125283B8F7D9425, 0x596C50E5678E3AA9347A393248A937F6, 0x527CDD75053A41E092CADF0A372F32F3, 0xC2A8AAFA1E45270316575D1C637C4679],
                        [0xDCB604D05890CBE19D289F9E6E971C27, 0x47F83DFB160259718BB80EDF3DFF02E4, 0x96B5A8B21C78401237D3D81E15DB89ED, 0xC7C05B117380AF9B19C3EB1C9B73CDEB, 0xFBEC9BB07817EF0EA49804518778FFB1, 0xD482C2D8BAD8687617DF9B7F775226C3, 0xCF893CDA5380238F63206A153AD54CCB, 0xED9F1A8A88966E4F0278CC36D918FB53],
                        [0x1A83AAC2764BC5FE4FF2B312732F2B30, 0x6BE5488D0D0F1CCD8740A2986B97E97A, 0xA4F2A20FF0BDF5E344D8DD3355BCFAB6, 0xBA258D210BB55E1EDFFEA2C75ED64723, 0x97551901F73A314241087FE175A9D8B8, 0x3A31B17E34A07F9194EAAEF3B4534FEC, 0x3BDED4CB0EAFD8E4AB7A50A16C49CF97, 0x9735C7FFF4967191A8053DEA1360C56A]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    print!("RSA_1024_u128: Recovered = [");
    for rr in recovered
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(recovered, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                        [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                        [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(recovered, message);

    // Example for RSA_2048_u128
    use cryptocol::asymmetric::RSA_2048_u128;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u128: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let cipher = rsa.encrypt_array_unit(&message);
    print!("RSA_2048_u128: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x839DAD116A1FCA0A88DE30958512746, 0xBFEF2D6407991B2673655BFB63C593A1, 0xFAE91CB14B6969DA6C4BFFD58602856F, 0x3A34449355FF5A2B487811E8CE340203, 0xCBF46CC4D1BB3CA2B6E7FC8E40933E17, 0xCEE3C80C904ED6EA71195B4A799D9F6, 0x16839F92C32D05797C396E59C50713C0, 0xF885BEAC66AB311232913210B7AACD80, 0xD55E6E53DC8E5EFA8E4B37AF75BC7B1A, 0x1CF6778B55F529D7AC8C176F55168D6A, 0x8733A73CFF1F2E96B828CD006927C346, 0x55EA897A87E4317B107A6AC1DCF3EC8B, 0x7A52100A72E3C18C7D109B38484B9F8C, 0xA0AAF25EEC16E847D2232D3E69574CD9, 0x1A7CC15644211691527275697812251E, 0x144B8CDE8D7CE45BF70A2641458B9D97],
                        [0x1A6F8D281CC9E10B8451F77AAF8EA34A, 0x46A8B16B759DCD636F52F1B1E535B, 0xFAAA44CBD47357D1694603D4A4DB4947, 0xB996F502E31648FD281FF56718A7D50A, 0x430A5970A3CBC1C61E1509C7F6FFC57F, 0x9AC886D16A104F83BDDE7C4B87501C03, 0x15689F6A781B31B4CAE6D365AFE5FCB3, 0x3E7EECD94281CB9AE042223EC393431, 0x41F39B10F3F0D946B958CB54ED46EEB9, 0xB6A22DD02CCB18C5C2152C2EDDAA89AF, 0xAA738CC74B40BCCAE4E9D89D4FEA1DC7, 0xE78C09629C4BF16830A41CA81FCF3CBD, 0x4957120044970C825FE30C7A96F42DE, 0x5DDA4CE5B6DBFB740C8880E2F4BEA26E, 0xB91E53CA72E227CAA6860B6E2AFE2CD0, 0x204ABDC606DFA0EFEC31BAC17DC37372],
                        [0x872DF0A7D7142C093E6622313EC97176, 0x4A6CBDE9E5ED5CDF9A2EB827912E04C9, 0x3D9005DDD5F05DD81EC68548AD9C195, 0xC7F81B10CC9A7BA5406BA4B03DB693E4, 0x5C3686F645CC9F4A78195A945B0FEEA7, 0xEB44EB696543C935CEA9D40FB17EA923, 0xED63CB323DEE373C464C27109C406F87, 0x38A4E18703160BC68DE9C7BAF784D1B4, 0x97047FAD40030B00F8486078D7C49411, 0xD68F4FED16279A66B00E34F5B837A89F, 0x8E1A882C2BB360C25ED4AED0470BD897, 0x64E737E55B8DFBA2DC1257F474A73D94, 0x69134CCD7C9AD0B4C7822C68395C8152, 0x4AA60B4C1101F3A44F72EDF6BD2535D6, 0xE11617FB0C85FADFF886D1E7BB03EC35, 0x10AA50CD90BD84BBBAD3992FAA005E06]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    
    print!("RSA_2048_u128: Recovered = [");
    for rr in recovered
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(recovered, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(recovered, message);

    // Example for RSA_4096_u128
    use cryptocol::asymmetric::RSA_4096_u128;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096_u128::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096_u128: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u128: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u128: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let cipher = rsa.encrypt_array_unit(&message);
    print!("RSA_4096_u128: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x50C6112C753A7FEE2C5F6411297D441C, 0x2E7B2CE2464B45EB0C967B739765F75A, 0xAA644C8B718D227DBE324F210AD8D127, 0x82F3C8945E688409B060802D478530AD, 0xC09E5C5CE9E5766FA2287A369930DB71, 0xA366D2415D195295DD452F9EB4DEBA1E, 0x27A5AA6F6BDE070EEDAB47C65C0E152E, 0xC4A9C2641919644655B1038D134009FB, 0x9EADB1FCF8EBC560A437DCD5AEC6AB51, 0xEF92D5BC205583266C10C632A446D423, 0xC40239A39BA2A06D61316515124F75C2, 0x8FFBEDA73BE3A38FFF04ED8B826440E8, 0xE799865E82A21E50F37A69D3BB154B25, 0x2E6B7BDFA92673B58DDA3FF8B9FC1E91, 0xEE7B7D26D1EC6253F10FFFAF2C6630AB, 0x7516FFA2821954B56233235AE367BC27, 0xE70F4930C6672059B9A7901E1BBF54E5, 0x2E39E8A149CDEBB8D4C1224339A64754, 0x31D7D3A7EC85450F94D3E2FCCAF382AD, 0x4F99EF5CF7C96DC1095BB0C42FC47057, 0xCF8D9BB4A1D4CD71A77AD0D64B14F7D7, 0x558B0A296118438B0367BD1B6F9EB898, 0x1F26303FA44449C83721EBBFC7E346CA, 0xABD140D0CD0F2DFF802240334205F7A3, 0xC09A4F193291E8D8D51B8A189AD41EDD, 0xBF50027C5BC8D0ECA6BDBAB9A53D31ED, 0x8960EBE0F1016DBF98A46C2ED8EFD4C7, 0x86B53F64C64D7A6B8C5F0A555783CE6E, 0xCB8C74E197A089B12267960685D2EF84, 0x389545E14F79D2D0EFFE6EC3A3CCC592, 0xBC113DF786CAF327DC0004BA84719E7C, 0xDD5831F77A8AD437D93A4A0D875A5C24],
                        [0xC992089EF20B52DFE551A4FEF8EA263C, 0x9D510616D7EF3A7875DC52BC74176FCA, 0x9D06E31C6FF6C7D6F44E4A6D76A595DC, 0xDCB6326C8E44C9F55B10CD8B1447F881, 0x62ACFF3EA9DAE98AD008BBB10B79F18C, 0x154367CF3041C333F774DD3D2A7C3CB7, 0x8EB7CE1A8DAD43590B7DE12F1DE4A90A, 0x29A2736502E3CC59A03E2ACFB11561CE, 0xA76CDC7CBDB3F787C186776C884BCF0C, 0xBBC4F265C308A18E341DB3CE32D0EC90, 0xCB2DCA030D4C2339BBE9D9308EA878C3, 0xDC6187A5851158E0F9086339AAA4361D, 0x587630DFD762704BB0DFC87FB85DC071, 0xCC6F76A4E72DEAB177702F19B0F3AFEE, 0x4AA72B7DE1803C31682257BBFBF91FE9, 0x166C0187B249A538D3B607D4319536E0, 0x209A5B74826266CD9F1089ABF121272D, 0xE0114D4CC89964C2698A875E51B7F6E4, 0xC92137E4632EA904FBFD50C3E32315E8, 0x79EFE6C6F50773CD48F7122CCCEE3076, 0xC4FE9C9DC5F23FDA1BA83BBCECFD11AC, 0x63DC6592F96D707DE8A94956DC6D0467, 0xDD82AC4E47306665800811FC9AE27FEC, 0x98896DD488E4F5FC8B81352836931217, 0x258B79A2DBA6B4826C19905AB80D2269, 0x72D0CB256C2845D2C1A514BD1A3C9AA5, 0x669BB63AD2D4D5170E76971718C1F440, 0x7A1123D0D5AA71B78761B91085737599, 0x60E991E272B71CA3A02A5025BCD95883, 0x70FCE4A92151FCA99176C80FAD6601C7, 0x2707FC2F7216959F483B834FA16B991C, 0x8DCA257D3D3CEFCC0EF3E168771A46E0],
                        [0xC198B82B42A27BDDDB4CE2DCF42BE309, 0x178A5D5F9EA7255B46FF7A3E3E7A2747, 0x6835F752E81D248F4EC09632089FFAAD, 0xDF30C11F2D0DF0B50C1DAFB0F6EC565D, 0x8D3EBEA720D05AC8D303EBF536B1ED61, 0x17127731C4A65B45015CC58A63D136F6, 0x3E2B0D84AEA46599011BB2E094F6B581, 0xDEC69E3ABD857C7BDAE2987D6D5CD0FC, 0x63D78F4E356C3B311D25D80255669B9A, 0xBCF6D009D0FAC27531ECD98855BA82A4, 0x73F0F16B7849CBF808394AD04D3BBE36, 0x62A5A6123C06EC6DE3BE5312D5ED807, 0x3E857E8232B7326DB85AB01697AD95D3, 0xC52CB9A4EF4AF832FE9085C1C2105D2E, 0xBE4C22A5AB41472DA7B30970F302F044, 0xCA9FFB1D5195A499D63C7428CFB4ACA8, 0xF4B0C737E7D11482047A7DDD7604D98C, 0x880981C0C51A71DDE39A51F785040FBC, 0x4823B42590DAFC99443873F4BF4CAF2F, 0x3A175AF4B033E79D5A01621485C04D13, 0xE01ACB15713C68FEDE0BE592AB6FBAD1, 0xDB21D751ECF206684B77433DE2410568, 0x1E137E49BC29F1CD2D9D62C6E80C456D, 0x48E555B77C589B17AC1D53D2ECC20CD9, 0x87EEC26716EA1D9D5D5EC0B188418FB0, 0x147902F627C1B230B9E2E216B83A7E17, 0x85B9DAACBCF186FA707C4CDE525D2FB5, 0xF1F2AE4C6D61C17C2DBB8CF95E9AFE8A, 0x98004AFB94820CC2FB5177CD1ECCC2BF, 0x9E2C9D40E6B0697D2ECD1A0EBE22490, 0xC4714130C4D07CC1DD9D50270B62101F, 0xD2114B52E1E238E6107E5ABF087F511B]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    
    print!("RSA_4096_u128: Recovered = [");
    for rr in recovered
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(recovered, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(recovered, message);

    // Example for RSA_1024_u64
    use cryptocol::asymmetric::RSA_1024_u64;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u64: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u64; 16]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr() as *mut u64, 16 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_1024_u64: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x86C53D7EE181EC1C, 0xC64EE28F0C1EE84B, 0xDCCF7F8DBCB1272C, 0xDEE7F8065CCEE57E, 0xEAF66FB4260E66E3, 0xF3331A7584C29D28, 0xAF51309B6A98F58E, 0x36D16E3D988A192A, 0xD125283B8F7D9425, 0xBA29353564A9AC13, 0x347A393248A937F6, 0x596C50E5678E3AA9, 0x92CADF0A372F32F3, 0x527CDD75053A41E0, 0x16575D1C637C4679, 0xC2A8AAFA1E452703],
                        [0x9D289F9E6E971C27, 0xDCB604D05890CBE1, 0x8BB80EDF3DFF02E4, 0x47F83DFB16025971, 0x37D3D81E15DB89ED, 0x96B5A8B21C784012, 0x19C3EB1C9B73CDEB, 0xC7C05B117380AF9B, 0xA49804518778FFB1, 0xFBEC9BB07817EF0E, 0x17DF9B7F775226C3, 0xD482C2D8BAD86876, 0x63206A153AD54CCB, 0xCF893CDA5380238F, 0x278CC36D918FB53, 0xED9F1A8A88966E4F],
                        [0x4FF2B312732F2B30, 0x1A83AAC2764BC5FE, 0x8740A2986B97E97A, 0x6BE5488D0D0F1CCD, 0x44D8DD3355BCFAB6, 0xA4F2A20FF0BDF5E3, 0xDFFEA2C75ED64723, 0xBA258D210BB55E1E, 0x41087FE175A9D8B8, 0x97551901F73A3142, 0x94EAAEF3B4534FEC, 0x3A31B17E34A07F91, 0xAB7A50A16C49CF97, 0x3BDED4CB0EAFD8E4, 0xA8053DEA1360C56A, 0x9735C7FFF4967191]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 8]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u64, rrr.as_mut_ptr() as *mut u64, 16 * 3); }
    
    print!("RSA_1024_u64: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(rrr, message);

    // Example for RSA_2048_u64
    use cryptocol::asymmetric::RSA_2048_u64;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u64: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u64; 32]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr() as *mut u64, 32 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_2048_u64: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xA88DE30958512746, 0x839DAD116A1FCA0, 0x73655BFB63C593A1, 0xBFEF2D6407991B26, 0x6C4BFFD58602856F, 0xFAE91CB14B6969DA, 0x487811E8CE340203, 0x3A34449355FF5A2B, 0xB6E7FC8E40933E17, 0xCBF46CC4D1BB3CA2, 0xA71195B4A799D9F6, 0xCEE3C80C904ED6E, 0x7C396E59C50713C0, 0x16839F92C32D0579, 0x32913210B7AACD80, 0xF885BEAC66AB3112, 0x8E4B37AF75BC7B1A, 0xD55E6E53DC8E5EFA, 0xAC8C176F55168D6A, 0x1CF6778B55F529D7, 0xB828CD006927C346, 0x8733A73CFF1F2E96, 0x107A6AC1DCF3EC8B, 0x55EA897A87E4317B, 0x7D109B38484B9F8C, 0x7A52100A72E3C18C, 0xD2232D3E69574CD9, 0xA0AAF25EEC16E847, 0x527275697812251E, 0x1A7CC15644211691, 0xF70A2641458B9D97, 0x144B8CDE8D7CE45B],
                        [0x8451F77AAF8EA34A, 0x1A6F8D281CC9E10B, 0xD636F52F1B1E535B, 0x46A8B16B759DC, 0x694603D4A4DB4947, 0xFAAA44CBD47357D1, 0x281FF56718A7D50A, 0xB996F502E31648FD, 0x1E1509C7F6FFC57F, 0x430A5970A3CBC1C6, 0xBDDE7C4B87501C03, 0x9AC886D16A104F83, 0xCAE6D365AFE5FCB3, 0x15689F6A781B31B4, 0xAE042223EC393431, 0x3E7EECD94281CB9, 0xB958CB54ED46EEB9, 0x41F39B10F3F0D946, 0xC2152C2EDDAA89AF, 0xB6A22DD02CCB18C5, 0xE4E9D89D4FEA1DC7, 0xAA738CC74B40BCCA, 0x30A41CA81FCF3CBD, 0xE78C09629C4BF168, 0x25FE30C7A96F42DE, 0x4957120044970C8, 0xC8880E2F4BEA26E, 0x5DDA4CE5B6DBFB74, 0xA6860B6E2AFE2CD0, 0xB91E53CA72E227CA, 0xEC31BAC17DC37372, 0x204ABDC606DFA0EF],
                        [0x3E6622313EC97176, 0x872DF0A7D7142C09, 0x9A2EB827912E04C9, 0x4A6CBDE9E5ED5CDF, 0x81EC68548AD9C195, 0x3D9005DDD5F05DD, 0x406BA4B03DB693E4, 0xC7F81B10CC9A7BA5, 0x78195A945B0FEEA7, 0x5C3686F645CC9F4A, 0xCEA9D40FB17EA923, 0xEB44EB696543C935, 0x464C27109C406F87, 0xED63CB323DEE373C, 0x8DE9C7BAF784D1B4, 0x38A4E18703160BC6, 0xF8486078D7C49411, 0x97047FAD40030B00, 0xB00E34F5B837A89F, 0xD68F4FED16279A66, 0x5ED4AED0470BD897, 0x8E1A882C2BB360C2, 0xDC1257F474A73D94, 0x64E737E55B8DFBA2, 0xC7822C68395C8152, 0x69134CCD7C9AD0B4, 0x4F72EDF6BD2535D6, 0x4AA60B4C1101F3A4, 0xF886D1E7BB03EC35, 0xE11617FB0C85FADF, 0xBAD3992FAA005E06, 0x10AA50CD90BD84BB]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 16]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u64, rrr.as_mut_ptr() as *mut u64, 32 * 3); }
    
    print!("RSA_2048_u64: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(rrr, message);

    // Example for RSA_4096_u64
    use cryptocol::asymmetric::RSA_4096_u64;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096_u64::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096_u64: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u64: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u64: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u64; 64]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u64, m.as_mut_ptr() as *mut u64, 64 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_4096_u64: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x2C5F6411297D441C, 0x50C6112C753A7FEE, 0xC967B739765F75A, 0x2E7B2CE2464B45EB, 0xBE324F210AD8D127, 0xAA644C8B718D227D, 0xB060802D478530AD, 0x82F3C8945E688409, 0xA2287A369930DB71, 0xC09E5C5CE9E5766F, 0xDD452F9EB4DEBA1E, 0xA366D2415D195295, 0xEDAB47C65C0E152E, 0x27A5AA6F6BDE070E, 0x55B1038D134009FB, 0xC4A9C26419196446, 0xA437DCD5AEC6AB51, 0x9EADB1FCF8EBC560, 0x6C10C632A446D423, 0xEF92D5BC20558326, 0x61316515124F75C2, 0xC40239A39BA2A06D, 0xFF04ED8B826440E8, 0x8FFBEDA73BE3A38F, 0xF37A69D3BB154B25, 0xE799865E82A21E50, 0x8DDA3FF8B9FC1E91, 0x2E6B7BDFA92673B5, 0xF10FFFAF2C6630AB, 0xEE7B7D26D1EC6253, 0x6233235AE367BC27, 0x7516FFA2821954B5, 0xB9A7901E1BBF54E5, 0xE70F4930C6672059, 0xD4C1224339A64754, 0x2E39E8A149CDEBB8, 0x94D3E2FCCAF382AD, 0x31D7D3A7EC85450F, 0x95BB0C42FC47057, 0x4F99EF5CF7C96DC1, 0xA77AD0D64B14F7D7, 0xCF8D9BB4A1D4CD71, 0x367BD1B6F9EB898, 0x558B0A296118438B, 0x3721EBBFC7E346CA, 0x1F26303FA44449C8, 0x802240334205F7A3, 0xABD140D0CD0F2DFF, 0xD51B8A189AD41EDD, 0xC09A4F193291E8D8, 0xA6BDBAB9A53D31ED, 0xBF50027C5BC8D0EC, 0x98A46C2ED8EFD4C7, 0x8960EBE0F1016DBF, 0x8C5F0A555783CE6E, 0x86B53F64C64D7A6B, 0x2267960685D2EF84, 0xCB8C74E197A089B1, 0xEFFE6EC3A3CCC592, 0x389545E14F79D2D0, 0xDC0004BA84719E7C, 0xBC113DF786CAF327, 0xD93A4A0D875A5C24, 0xDD5831F77A8AD437],
                        [0xE551A4FEF8EA263C, 0xC992089EF20B52DF, 0x75DC52BC74176FCA, 0x9D510616D7EF3A78, 0xF44E4A6D76A595DC, 0x9D06E31C6FF6C7D6, 0x5B10CD8B1447F881, 0xDCB6326C8E44C9F5, 0xD008BBB10B79F18C, 0x62ACFF3EA9DAE98A, 0xF774DD3D2A7C3CB7, 0x154367CF3041C333, 0xB7DE12F1DE4A90A, 0x8EB7CE1A8DAD4359, 0xA03E2ACFB11561CE, 0x29A2736502E3CC59, 0xC186776C884BCF0C, 0xA76CDC7CBDB3F787, 0x341DB3CE32D0EC90, 0xBBC4F265C308A18E, 0xBBE9D9308EA878C3, 0xCB2DCA030D4C2339, 0xF9086339AAA4361D, 0xDC6187A5851158E0, 0xB0DFC87FB85DC071, 0x587630DFD762704B, 0x77702F19B0F3AFEE, 0xCC6F76A4E72DEAB1, 0x682257BBFBF91FE9, 0x4AA72B7DE1803C31, 0xD3B607D4319536E0, 0x166C0187B249A538, 0x9F1089ABF121272D, 0x209A5B74826266CD, 0x698A875E51B7F6E4, 0xE0114D4CC89964C2, 0xFBFD50C3E32315E8, 0xC92137E4632EA904, 0x48F7122CCCEE3076, 0x79EFE6C6F50773CD, 0x1BA83BBCECFD11AC, 0xC4FE9C9DC5F23FDA, 0xE8A94956DC6D0467, 0x63DC6592F96D707D, 0x800811FC9AE27FEC, 0xDD82AC4E47306665, 0x8B81352836931217, 0x98896DD488E4F5FC, 0x6C19905AB80D2269, 0x258B79A2DBA6B482, 0xC1A514BD1A3C9AA5, 0x72D0CB256C2845D2, 0xE76971718C1F440, 0x669BB63AD2D4D517, 0x8761B91085737599, 0x7A1123D0D5AA71B7, 0xA02A5025BCD95883, 0x60E991E272B71CA3, 0x9176C80FAD6601C7, 0x70FCE4A92151FCA9, 0x483B834FA16B991C, 0x2707FC2F7216959F, 0xEF3E168771A46E0, 0x8DCA257D3D3CEFCC],
                        [0xDB4CE2DCF42BE309, 0xC198B82B42A27BDD, 0x46FF7A3E3E7A2747, 0x178A5D5F9EA7255B, 0x4EC09632089FFAAD, 0x6835F752E81D248F, 0xC1DAFB0F6EC565D, 0xDF30C11F2D0DF0B5, 0xD303EBF536B1ED61, 0x8D3EBEA720D05AC8, 0x15CC58A63D136F6, 0x17127731C4A65B45, 0x11BB2E094F6B581, 0x3E2B0D84AEA46599, 0xDAE2987D6D5CD0FC, 0xDEC69E3ABD857C7B, 0x1D25D80255669B9A, 0x63D78F4E356C3B31, 0x31ECD98855BA82A4, 0xBCF6D009D0FAC275, 0x8394AD04D3BBE36, 0x73F0F16B7849CBF8, 0xDE3BE5312D5ED807, 0x62A5A6123C06EC6, 0xB85AB01697AD95D3, 0x3E857E8232B7326D, 0xFE9085C1C2105D2E, 0xC52CB9A4EF4AF832, 0xA7B30970F302F044, 0xBE4C22A5AB41472D, 0xD63C7428CFB4ACA8, 0xCA9FFB1D5195A499, 0x47A7DDD7604D98C, 0xF4B0C737E7D11482, 0xE39A51F785040FBC, 0x880981C0C51A71DD, 0x443873F4BF4CAF2F, 0x4823B42590DAFC99, 0x5A01621485C04D13, 0x3A175AF4B033E79D, 0xDE0BE592AB6FBAD1, 0xE01ACB15713C68FE, 0x4B77433DE2410568, 0xDB21D751ECF20668, 0x2D9D62C6E80C456D, 0x1E137E49BC29F1CD, 0xAC1D53D2ECC20CD9, 0x48E555B77C589B17, 0x5D5EC0B188418FB0, 0x87EEC26716EA1D9D, 0xB9E2E216B83A7E17, 0x147902F627C1B230, 0x707C4CDE525D2FB5, 0x85B9DAACBCF186FA, 0x2DBB8CF95E9AFE8A, 0xF1F2AE4C6D61C17C, 0xFB5177CD1ECCC2BF, 0x98004AFB94820CC2, 0xD2ECD1A0EBE22490, 0x9E2C9D40E6B0697, 0xDD9D50270B62101F, 0xC4714130C4D07CC1, 0x107E5ABF087F511B, 0xD2114B52E1E238E6]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 32]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u64, rrr.as_mut_ptr() as *mut u64, 64 * 3); }
    
    print!("RSA_4096_u64: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(rrr, message);

    // Example for RSA_1024_u32
    use cryptocol::asymmetric::RSA_1024_u32;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u32: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 32]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 32 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_1024_u32: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xE181EC1C, 0x86C53D7E, 0xC1EE84B, 0xC64EE28F, 0xBCB1272C, 0xDCCF7F8D, 0x5CCEE57E, 0xDEE7F806, 0x260E66E3, 0xEAF66FB4, 0x84C29D28, 0xF3331A75, 0x6A98F58E, 0xAF51309B, 0x988A192A, 0x36D16E3D, 0x8F7D9425, 0xD125283B, 0x64A9AC13, 0xBA293535, 0x48A937F6, 0x347A3932, 0x678E3AA9, 0x596C50E5, 0x372F32F3, 0x92CADF0A, 0x53A41E0, 0x527CDD75, 0x637C4679, 0x16575D1C, 0x1E452703, 0xC2A8AAFA],
                        [0x6E971C27, 0x9D289F9E, 0x5890CBE1, 0xDCB604D0, 0x3DFF02E4, 0x8BB80EDF, 0x16025971, 0x47F83DFB, 0x15DB89ED, 0x37D3D81E, 0x1C784012, 0x96B5A8B2, 0x9B73CDEB, 0x19C3EB1C, 0x7380AF9B, 0xC7C05B11, 0x8778FFB1, 0xA4980451, 0x7817EF0E, 0xFBEC9BB0, 0x775226C3, 0x17DF9B7F, 0xBAD86876, 0xD482C2D8, 0x3AD54CCB, 0x63206A15, 0x5380238F, 0xCF893CDA, 0xD918FB53, 0x278CC36, 0x88966E4F, 0xED9F1A8A],
                        [0x732F2B30, 0x4FF2B312, 0x764BC5FE, 0x1A83AAC2, 0x6B97E97A, 0x8740A298, 0xD0F1CCD, 0x6BE5488D, 0x55BCFAB6, 0x44D8DD33, 0xF0BDF5E3, 0xA4F2A20F, 0x5ED64723, 0xDFFEA2C7, 0xBB55E1E, 0xBA258D21, 0x75A9D8B8, 0x41087FE1, 0xF73A3142, 0x97551901, 0xB4534FEC, 0x94EAAEF3, 0x34A07F91, 0x3A31B17E, 0x6C49CF97, 0xAB7A50A1, 0xEAFD8E4, 0x3BDED4CB, 0x1360C56A, 0xA8053DEA, 0xF4967191, 0x9735C7FF]]);
    let mut rrr = [[0u128; 8]; 3];
    let recovered = rsa.decrypt_array_unit(&cipher);
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 32 * 3); }
    
    print!("RSA_1024_u32: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(rrr, message);

    // Example for RSA_2048_u32
    use cryptocol::asymmetric::RSA_2048_u32;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u32: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 64]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 64 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_2048_u32: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x58512746, 0xA88DE309, 0x16A1FCA0, 0x839DAD1, 0x63C593A1, 0x73655BFB, 0x7991B26, 0xBFEF2D64, 0x8602856F, 0x6C4BFFD5, 0x4B6969DA, 0xFAE91CB1, 0xCE340203, 0x487811E8, 0x55FF5A2B, 0x3A344493, 0x40933E17, 0xB6E7FC8E, 0xD1BB3CA2, 0xCBF46CC4, 0xA799D9F6, 0xA71195B4, 0xC904ED6E, 0xCEE3C80, 0xC50713C0, 0x7C396E59, 0xC32D0579, 0x16839F92, 0xB7AACD80, 0x32913210, 0x66AB3112, 0xF885BEAC, 0x75BC7B1A, 0x8E4B37AF, 0xDC8E5EFA, 0xD55E6E53, 0x55168D6A, 0xAC8C176F, 0x55F529D7, 0x1CF6778B, 0x6927C346, 0xB828CD00, 0xFF1F2E96, 0x8733A73C, 0xDCF3EC8B, 0x107A6AC1, 0x87E4317B, 0x55EA897A, 0x484B9F8C, 0x7D109B38, 0x72E3C18C, 0x7A52100A, 0x69574CD9, 0xD2232D3E, 0xEC16E847, 0xA0AAF25E, 0x7812251E, 0x52727569, 0x44211691, 0x1A7CC156, 0x458B9D97, 0xF70A2641, 0x8D7CE45B, 0x144B8CDE],
                        [0xAF8EA34A, 0x8451F77A, 0x1CC9E10B, 0x1A6F8D28, 0x1B1E535B, 0xD636F52F, 0x16B759DC, 0x46A8B, 0xA4DB4947, 0x694603D4, 0xD47357D1, 0xFAAA44CB, 0x18A7D50A, 0x281FF567, 0xE31648FD, 0xB996F502, 0xF6FFC57F, 0x1E1509C7, 0xA3CBC1C6, 0x430A5970, 0x87501C03, 0xBDDE7C4B, 0x6A104F83, 0x9AC886D1, 0xAFE5FCB3, 0xCAE6D365, 0x781B31B4, 0x15689F6A, 0xEC393431, 0xAE042223, 0x94281CB9, 0x3E7EECD, 0xED46EEB9, 0xB958CB54, 0xF3F0D946, 0x41F39B10, 0xDDAA89AF, 0xC2152C2E, 0x2CCB18C5, 0xB6A22DD0, 0x4FEA1DC7, 0xE4E9D89D, 0x4B40BCCA, 0xAA738CC7, 0x1FCF3CBD, 0x30A41CA8, 0x9C4BF168, 0xE78C0962, 0xA96F42DE, 0x25FE30C7, 0x44970C8, 0x4957120, 0xF4BEA26E, 0xC8880E2, 0xB6DBFB74, 0x5DDA4CE5, 0x2AFE2CD0, 0xA6860B6E, 0x72E227CA, 0xB91E53CA, 0x7DC37372, 0xEC31BAC1, 0x6DFA0EF, 0x204ABDC6],
                        [0x3EC97176, 0x3E662231, 0xD7142C09, 0x872DF0A7, 0x912E04C9, 0x9A2EB827, 0xE5ED5CDF, 0x4A6CBDE9, 0x8AD9C195, 0x81EC6854, 0xDD5F05DD, 0x3D9005D, 0x3DB693E4, 0x406BA4B0, 0xCC9A7BA5, 0xC7F81B10, 0x5B0FEEA7, 0x78195A94, 0x45CC9F4A, 0x5C3686F6, 0xB17EA923, 0xCEA9D40F, 0x6543C935, 0xEB44EB69, 0x9C406F87, 0x464C2710, 0x3DEE373C, 0xED63CB32, 0xF784D1B4, 0x8DE9C7BA, 0x3160BC6, 0x38A4E187, 0xD7C49411, 0xF8486078, 0x40030B00, 0x97047FAD, 0xB837A89F, 0xB00E34F5, 0x16279A66, 0xD68F4FED, 0x470BD897, 0x5ED4AED0, 0x2BB360C2, 0x8E1A882C, 0x74A73D94, 0xDC1257F4, 0x5B8DFBA2, 0x64E737E5, 0x395C8152, 0xC7822C68, 0x7C9AD0B4, 0x69134CCD, 0xBD2535D6, 0x4F72EDF6, 0x1101F3A4, 0x4AA60B4C, 0xBB03EC35, 0xF886D1E7, 0xC85FADF, 0xE11617FB, 0xAA005E06, 0xBAD3992F, 0x90BD84BB, 0x10AA50CD]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 16]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 64 * 3); }
    
    print!("RSA_2048_u32: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(rrr, message);

    // Example for RSA_4096_u32
    use cryptocol::asymmetric::RSA_4096_u32;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096_u32::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096_u32: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u32: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u32: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u32; 128]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 128 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_4096_u32: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x297D441C, 0x2C5F6411, 0x753A7FEE, 0x50C6112C, 0x9765F75A, 0xC967B73, 0x464B45EB, 0x2E7B2CE2, 0xAD8D127, 0xBE324F21, 0x718D227D, 0xAA644C8B, 0x478530AD, 0xB060802D, 0x5E688409, 0x82F3C894, 0x9930DB71, 0xA2287A36, 0xE9E5766F, 0xC09E5C5C, 0xB4DEBA1E, 0xDD452F9E, 0x5D195295, 0xA366D241, 0x5C0E152E, 0xEDAB47C6, 0x6BDE070E, 0x27A5AA6F, 0x134009FB, 0x55B1038D, 0x19196446, 0xC4A9C264, 0xAEC6AB51, 0xA437DCD5, 0xF8EBC560, 0x9EADB1FC, 0xA446D423, 0x6C10C632, 0x20558326, 0xEF92D5BC, 0x124F75C2, 0x61316515, 0x9BA2A06D, 0xC40239A3, 0x826440E8, 0xFF04ED8B, 0x3BE3A38F, 0x8FFBEDA7, 0xBB154B25, 0xF37A69D3, 0x82A21E50, 0xE799865E, 0xB9FC1E91, 0x8DDA3FF8, 0xA92673B5, 0x2E6B7BDF, 0x2C6630AB, 0xF10FFFAF, 0xD1EC6253, 0xEE7B7D26, 0xE367BC27, 0x6233235A, 0x821954B5, 0x7516FFA2, 0x1BBF54E5, 0xB9A7901E, 0xC6672059, 0xE70F4930, 0x39A64754, 0xD4C12243, 0x49CDEBB8, 0x2E39E8A1, 0xCAF382AD, 0x94D3E2FC, 0xEC85450F, 0x31D7D3A7, 0x2FC47057, 0x95BB0C4, 0xF7C96DC1, 0x4F99EF5C, 0x4B14F7D7, 0xA77AD0D6, 0xA1D4CD71, 0xCF8D9BB4, 0x6F9EB898, 0x367BD1B, 0x6118438B, 0x558B0A29, 0xC7E346CA, 0x3721EBBF, 0xA44449C8, 0x1F26303F, 0x4205F7A3, 0x80224033, 0xCD0F2DFF, 0xABD140D0, 0x9AD41EDD, 0xD51B8A18, 0x3291E8D8, 0xC09A4F19, 0xA53D31ED, 0xA6BDBAB9, 0x5BC8D0EC, 0xBF50027C, 0xD8EFD4C7, 0x98A46C2E, 0xF1016DBF, 0x8960EBE0, 0x5783CE6E, 0x8C5F0A55, 0xC64D7A6B, 0x86B53F64, 0x85D2EF84, 0x22679606, 0x97A089B1, 0xCB8C74E1, 0xA3CCC592, 0xEFFE6EC3, 0x4F79D2D0, 0x389545E1, 0x84719E7C, 0xDC0004BA, 0x86CAF327, 0xBC113DF7, 0x875A5C24, 0xD93A4A0D, 0x7A8AD437, 0xDD5831F7],
                        [0xF8EA263C, 0xE551A4FE, 0xF20B52DF, 0xC992089E, 0x74176FCA, 0x75DC52BC, 0xD7EF3A78, 0x9D510616, 0x76A595DC, 0xF44E4A6D, 0x6FF6C7D6, 0x9D06E31C, 0x1447F881, 0x5B10CD8B, 0x8E44C9F5, 0xDCB6326C, 0xB79F18C, 0xD008BBB1, 0xA9DAE98A, 0x62ACFF3E, 0x2A7C3CB7, 0xF774DD3D, 0x3041C333, 0x154367CF, 0x1DE4A90A, 0xB7DE12F, 0x8DAD4359, 0x8EB7CE1A, 0xB11561CE, 0xA03E2ACF, 0x2E3CC59, 0x29A27365, 0x884BCF0C, 0xC186776C, 0xBDB3F787, 0xA76CDC7C, 0x32D0EC90, 0x341DB3CE, 0xC308A18E, 0xBBC4F265, 0x8EA878C3, 0xBBE9D930, 0xD4C2339, 0xCB2DCA03, 0xAAA4361D, 0xF9086339, 0x851158E0, 0xDC6187A5, 0xB85DC071, 0xB0DFC87F, 0xD762704B, 0x587630DF, 0xB0F3AFEE, 0x77702F19, 0xE72DEAB1, 0xCC6F76A4, 0xFBF91FE9, 0x682257BB, 0xE1803C31, 0x4AA72B7D, 0x319536E0, 0xD3B607D4, 0xB249A538, 0x166C0187, 0xF121272D, 0x9F1089AB, 0x826266CD, 0x209A5B74, 0x51B7F6E4, 0x698A875E, 0xC89964C2, 0xE0114D4C, 0xE32315E8, 0xFBFD50C3, 0x632EA904, 0xC92137E4, 0xCCEE3076, 0x48F7122C, 0xF50773CD, 0x79EFE6C6, 0xECFD11AC, 0x1BA83BBC, 0xC5F23FDA, 0xC4FE9C9D, 0xDC6D0467, 0xE8A94956, 0xF96D707D, 0x63DC6592, 0x9AE27FEC, 0x800811FC, 0x47306665, 0xDD82AC4E, 0x36931217, 0x8B813528, 0x88E4F5FC, 0x98896DD4, 0xB80D2269, 0x6C19905A, 0xDBA6B482, 0x258B79A2, 0x1A3C9AA5, 0xC1A514BD, 0x6C2845D2, 0x72D0CB25, 0x18C1F440, 0xE769717, 0xD2D4D517, 0x669BB63A, 0x85737599, 0x8761B910, 0xD5AA71B7, 0x7A1123D0, 0xBCD95883, 0xA02A5025, 0x72B71CA3, 0x60E991E2, 0xAD6601C7, 0x9176C80F, 0x2151FCA9, 0x70FCE4A9, 0xA16B991C, 0x483B834F, 0x7216959F, 0x2707FC2F, 0x771A46E0, 0xEF3E168, 0x3D3CEFCC, 0x8DCA257D],
                        [0xF42BE309, 0xDB4CE2DC, 0x42A27BDD, 0xC198B82B, 0x3E7A2747, 0x46FF7A3E, 0x9EA7255B, 0x178A5D5F, 0x89FFAAD, 0x4EC09632, 0xE81D248F, 0x6835F752, 0xF6EC565D, 0xC1DAFB0, 0x2D0DF0B5, 0xDF30C11F, 0x36B1ED61, 0xD303EBF5, 0x20D05AC8, 0x8D3EBEA7, 0x63D136F6, 0x15CC58A, 0xC4A65B45, 0x17127731, 0x94F6B581, 0x11BB2E0, 0xAEA46599, 0x3E2B0D84, 0x6D5CD0FC, 0xDAE2987D, 0xBD857C7B, 0xDEC69E3A, 0x55669B9A, 0x1D25D802, 0x356C3B31, 0x63D78F4E, 0x55BA82A4, 0x31ECD988, 0xD0FAC275, 0xBCF6D009, 0x4D3BBE36, 0x8394AD0, 0x7849CBF8, 0x73F0F16B, 0x2D5ED807, 0xDE3BE531, 0x23C06EC6, 0x62A5A61, 0x97AD95D3, 0xB85AB016, 0x32B7326D, 0x3E857E82, 0xC2105D2E, 0xFE9085C1, 0xEF4AF832, 0xC52CB9A4, 0xF302F044, 0xA7B30970, 0xAB41472D, 0xBE4C22A5, 0xCFB4ACA8, 0xD63C7428, 0x5195A499, 0xCA9FFB1D, 0x7604D98C, 0x47A7DDD, 0xE7D11482, 0xF4B0C737, 0x85040FBC, 0xE39A51F7, 0xC51A71DD, 0x880981C0, 0xBF4CAF2F, 0x443873F4, 0x90DAFC99, 0x4823B425, 0x85C04D13, 0x5A016214, 0xB033E79D, 0x3A175AF4, 0xAB6FBAD1, 0xDE0BE592, 0x713C68FE, 0xE01ACB15, 0xE2410568, 0x4B77433D, 0xECF20668, 0xDB21D751, 0xE80C456D, 0x2D9D62C6, 0xBC29F1CD, 0x1E137E49, 0xECC20CD9, 0xAC1D53D2, 0x7C589B17, 0x48E555B7, 0x88418FB0, 0x5D5EC0B1, 0x16EA1D9D, 0x87EEC267, 0xB83A7E17, 0xB9E2E216, 0x27C1B230, 0x147902F6, 0x525D2FB5, 0x707C4CDE, 0xBCF186FA, 0x85B9DAAC, 0x5E9AFE8A, 0x2DBB8CF9, 0x6D61C17C, 0xF1F2AE4C, 0x1ECCC2BF, 0xFB5177CD, 0x94820CC2, 0x98004AFB, 0xEBE22490, 0xD2ECD1A0, 0xE6B0697, 0x9E2C9D4, 0xB62101F, 0xDD9D5027, 0xC4D07CC1, 0xC4714130, 0x87F511B, 0x107E5ABF, 0xE1E238E6, 0xD2114B52]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 32]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 128 * 3); }
    
    print!("RSA_4096_u32: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(rrr, message);

    // Example for RSA_1024_u16
    use cryptocol::asymmetric::RSA_1024_u16;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u16: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u16; 64]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr() as *mut u16, 64 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_1024_u16: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0xEC1C, 0xE181, 0x3D7E, 0x86C5, 0xE84B, 0xC1E, 0xE28F, 0xC64E, 0x272C, 0xBCB1, 0x7F8D, 0xDCCF, 0xE57E, 0x5CCE, 0xF806, 0xDEE7, 0x66E3, 0x260E, 0x6FB4, 0xEAF6, 0x9D28, 0x84C2, 0x1A75, 0xF333, 0xF58E, 0x6A98, 0x309B, 0xAF51, 0x192A, 0x988A, 0x6E3D, 0x36D1, 0x9425, 0x8F7D, 0x283B, 0xD125, 0xAC13, 0x64A9, 0x3535, 0xBA29, 0x37F6, 0x48A9, 0x3932, 0x347A, 0x3AA9, 0x678E, 0x50E5, 0x596C, 0x32F3, 0x372F, 0xDF0A, 0x92CA, 0x41E0, 0x53A, 0xDD75, 0x527C, 0x4679, 0x637C, 0x5D1C, 0x1657, 0x2703, 0x1E45, 0xAAFA, 0xC2A8],
                        [0x1C27, 0x6E97, 0x9F9E, 0x9D28, 0xCBE1, 0x5890, 0x4D0, 0xDCB6, 0x2E4, 0x3DFF, 0xEDF, 0x8BB8, 0x5971, 0x1602, 0x3DFB, 0x47F8, 0x89ED, 0x15DB, 0xD81E, 0x37D3, 0x4012, 0x1C78, 0xA8B2, 0x96B5, 0xCDEB, 0x9B73, 0xEB1C, 0x19C3, 0xAF9B, 0x7380, 0x5B11, 0xC7C0, 0xFFB1, 0x8778, 0x451, 0xA498, 0xEF0E, 0x7817, 0x9BB0, 0xFBEC, 0x26C3, 0x7752, 0x9B7F, 0x17DF, 0x6876, 0xBAD8, 0xC2D8, 0xD482, 0x4CCB, 0x3AD5, 0x6A15, 0x6320, 0x238F, 0x5380, 0x3CDA, 0xCF89, 0xFB53, 0xD918, 0xCC36, 0x278, 0x6E4F, 0x8896, 0x1A8A, 0xED9F],
                        [0x2B30, 0x732F, 0xB312, 0x4FF2, 0xC5FE, 0x764B, 0xAAC2, 0x1A83, 0xE97A, 0x6B97, 0xA298, 0x8740, 0x1CCD, 0xD0F, 0x488D, 0x6BE5, 0xFAB6, 0x55BC, 0xDD33, 0x44D8, 0xF5E3, 0xF0BD, 0xA20F, 0xA4F2, 0x4723, 0x5ED6, 0xA2C7, 0xDFFE, 0x5E1E, 0xBB5, 0x8D21, 0xBA25, 0xD8B8, 0x75A9, 0x7FE1, 0x4108, 0x3142, 0xF73A, 0x1901, 0x9755, 0x4FEC, 0xB453, 0xAEF3, 0x94EA, 0x7F91, 0x34A0, 0xB17E, 0x3A31, 0xCF97, 0x6C49, 0x50A1, 0xAB7A, 0xD8E4, 0xEAF, 0xD4CB, 0x3BDE, 0xC56A, 0x1360, 0x3DEA, 0xA805, 0x7191, 0xF496, 0xC7FF, 0x9735]]);
    let mut rrr = [[0u128; 8]; 3];
    let recovered = rsa.decrypt_array_unit(&cipher);
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u16, rrr.as_mut_ptr() as *mut u16, 64 * 3); }
    
    print!("RSA_1024_u16: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(rrr, message);

    // Example for RSA_2048_u16
    use cryptocol::asymmetric::RSA_2048_u16;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u16: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u16; 128]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr() as *mut u16, 128 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_2048_u16: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x2746, 0x5851, 0xE309, 0xA88D, 0xFCA0, 0x16A1, 0xDAD1, 0x839, 0x93A1, 0x63C5, 0x5BFB, 0x7365, 0x1B26, 0x799, 0x2D64, 0xBFEF, 0x856F, 0x8602, 0xFFD5, 0x6C4B, 0x69DA, 0x4B69, 0x1CB1, 0xFAE9, 0x203, 0xCE34, 0x11E8, 0x4878, 0x5A2B, 0x55FF, 0x4493, 0x3A34, 0x3E17, 0x4093, 0xFC8E, 0xB6E7, 0x3CA2, 0xD1BB, 0x6CC4, 0xCBF4, 0xD9F6, 0xA799, 0x95B4, 0xA711, 0xED6E, 0xC904, 0x3C80, 0xCEE, 0x13C0, 0xC507, 0x6E59, 0x7C39, 0x579, 0xC32D, 0x9F92, 0x1683, 0xCD80, 0xB7AA, 0x3210, 0x3291, 0x3112, 0x66AB, 0xBEAC, 0xF885, 0x7B1A, 0x75BC, 0x37AF, 0x8E4B, 0x5EFA, 0xDC8E, 0x6E53, 0xD55E, 0x8D6A, 0x5516, 0x176F, 0xAC8C, 0x29D7, 0x55F5, 0x778B, 0x1CF6, 0xC346, 0x6927, 0xCD00, 0xB828, 0x2E96, 0xFF1F, 0xA73C, 0x8733, 0xEC8B, 0xDCF3, 0x6AC1, 0x107A, 0x317B, 0x87E4, 0x897A, 0x55EA, 0x9F8C, 0x484B, 0x9B38, 0x7D10, 0xC18C, 0x72E3, 0x100A, 0x7A52, 0x4CD9, 0x6957, 0x2D3E, 0xD223, 0xE847, 0xEC16, 0xF25E, 0xA0AA, 0x251E, 0x7812, 0x7569, 0x5272, 0x1691, 0x4421, 0xC156, 0x1A7C, 0x9D97, 0x458B, 0x2641, 0xF70A, 0xE45B, 0x8D7C, 0x8CDE, 0x144B],
                        [0xA34A, 0xAF8E, 0xF77A, 0x8451, 0xE10B, 0x1CC9, 0x8D28, 0x1A6F, 0x535B, 0x1B1E, 0xF52F, 0xD636, 0x59DC, 0x16B7, 0x6A8B, 0x4, 0x4947, 0xA4DB, 0x3D4, 0x6946, 0x57D1, 0xD473, 0x44CB, 0xFAAA, 0xD50A, 0x18A7, 0xF567, 0x281F, 0x48FD, 0xE316, 0xF502, 0xB996, 0xC57F, 0xF6FF, 0x9C7, 0x1E15, 0xC1C6, 0xA3CB, 0x5970, 0x430A, 0x1C03, 0x8750, 0x7C4B, 0xBDDE, 0x4F83, 0x6A10, 0x86D1, 0x9AC8, 0xFCB3, 0xAFE5, 0xD365, 0xCAE6, 0x31B4, 0x781B, 0x9F6A, 0x1568, 0x3431, 0xEC39, 0x2223, 0xAE04, 0x1CB9, 0x9428, 0xEECD, 0x3E7, 0xEEB9, 0xED46, 0xCB54, 0xB958, 0xD946, 0xF3F0, 0x9B10, 0x41F3, 0x89AF, 0xDDAA, 0x2C2E, 0xC215, 0x18C5, 0x2CCB, 0x2DD0, 0xB6A2, 0x1DC7, 0x4FEA, 0xD89D, 0xE4E9, 0xBCCA, 0x4B40, 0x8CC7, 0xAA73, 0x3CBD, 0x1FCF, 0x1CA8, 0x30A4, 0xF168, 0x9C4B, 0x962, 0xE78C, 0x42DE, 0xA96F, 0x30C7, 0x25FE, 0x70C8, 0x449, 0x7120, 0x495, 0xA26E, 0xF4BE, 0x80E2, 0xC88, 0xFB74, 0xB6DB, 0x4CE5, 0x5DDA, 0x2CD0, 0x2AFE, 0xB6E, 0xA686, 0x27CA, 0x72E2, 0x53CA, 0xB91E, 0x7372, 0x7DC3, 0xBAC1, 0xEC31, 0xA0EF, 0x6DF, 0xBDC6, 0x204A],
                        [0x7176, 0x3EC9, 0x2231, 0x3E66, 0x2C09, 0xD714, 0xF0A7, 0x872D, 0x4C9, 0x912E, 0xB827, 0x9A2E, 0x5CDF, 0xE5ED, 0xBDE9, 0x4A6C, 0xC195, 0x8AD9, 0x6854, 0x81EC, 0x5DD, 0xDD5F, 0x5D, 0x3D9, 0x93E4, 0x3DB6, 0xA4B0, 0x406B, 0x7BA5, 0xCC9A, 0x1B10, 0xC7F8, 0xEEA7, 0x5B0F, 0x5A94, 0x7819, 0x9F4A, 0x45CC, 0x86F6, 0x5C36, 0xA923, 0xB17E, 0xD40F, 0xCEA9, 0xC935, 0x6543, 0xEB69, 0xEB44, 0x6F87, 0x9C40, 0x2710, 0x464C, 0x373C, 0x3DEE, 0xCB32, 0xED63, 0xD1B4, 0xF784, 0xC7BA, 0x8DE9, 0xBC6, 0x316, 0xE187, 0x38A4, 0x9411, 0xD7C4, 0x6078, 0xF848, 0xB00, 0x4003, 0x7FAD, 0x9704, 0xA89F, 0xB837, 0x34F5, 0xB00E, 0x9A66, 0x1627, 0x4FED, 0xD68F, 0xD897, 0x470B, 0xAED0, 0x5ED4, 0x60C2, 0x2BB3, 0x882C, 0x8E1A, 0x3D94, 0x74A7, 0x57F4, 0xDC12, 0xFBA2, 0x5B8D, 0x37E5, 0x64E7, 0x8152, 0x395C, 0x2C68, 0xC782, 0xD0B4, 0x7C9A, 0x4CCD, 0x6913, 0x35D6, 0xBD25, 0xEDF6, 0x4F72, 0xF3A4, 0x1101, 0xB4C, 0x4AA6, 0xEC35, 0xBB03, 0xD1E7, 0xF886, 0xFADF, 0xC85, 0x17FB, 0xE116, 0x5E06, 0xAA00, 0x992F, 0xBAD3, 0x84BB, 0x90BD, 0x50CD, 0x10AA]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 16]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u16, rrr.as_mut_ptr() as *mut u16, 128 * 3); }
    
    print!("RSA_2048_u16: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(rrr, message);

    // Example for RSA_4096_u16
    use cryptocol::asymmetric::RSA_4096_u16;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096_u16::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u16: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u16; 256]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr() as *mut u16, 256 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_4096_u16: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x441C, 0x297D, 0x6411, 0x2C5F, 0x7FEE, 0x753A, 0x112C, 0x50C6, 0xF75A, 0x9765, 0x7B73, 0xC96, 0x45EB, 0x464B, 0x2CE2, 0x2E7B, 0xD127, 0xAD8, 0x4F21, 0xBE32, 0x227D, 0x718D, 0x4C8B, 0xAA64, 0x30AD, 0x4785, 0x802D, 0xB060, 0x8409, 0x5E68, 0xC894, 0x82F3, 0xDB71, 0x9930, 0x7A36, 0xA228, 0x766F, 0xE9E5, 0x5C5C, 0xC09E, 0xBA1E, 0xB4DE, 0x2F9E, 0xDD45, 0x5295, 0x5D19, 0xD241, 0xA366, 0x152E, 0x5C0E, 0x47C6, 0xEDAB, 0x70E, 0x6BDE, 0xAA6F, 0x27A5, 0x9FB, 0x1340, 0x38D, 0x55B1, 0x6446, 0x1919, 0xC264, 0xC4A9, 0xAB51, 0xAEC6, 0xDCD5, 0xA437, 0xC560, 0xF8EB, 0xB1FC, 0x9EAD, 0xD423, 0xA446, 0xC632, 0x6C10, 0x8326, 0x2055, 0xD5BC, 0xEF92, 0x75C2, 0x124F, 0x6515, 0x6131, 0xA06D, 0x9BA2, 0x39A3, 0xC402, 0x40E8, 0x8264, 0xED8B, 0xFF04, 0xA38F, 0x3BE3, 0xEDA7, 0x8FFB, 0x4B25, 0xBB15, 0x69D3, 0xF37A, 0x1E50, 0x82A2, 0x865E, 0xE799, 0x1E91, 0xB9FC, 0x3FF8, 0x8DDA, 0x73B5, 0xA926, 0x7BDF, 0x2E6B, 0x30AB, 0x2C66, 0xFFAF, 0xF10F, 0x6253, 0xD1EC, 0x7D26, 0xEE7B, 0xBC27, 0xE367, 0x235A, 0x6233, 0x54B5, 0x8219, 0xFFA2, 0x7516, 0x54E5, 0x1BBF, 0x901E, 0xB9A7, 0x2059, 0xC667, 0x4930, 0xE70F, 0x4754, 0x39A6, 0x2243, 0xD4C1, 0xEBB8, 0x49CD, 0xE8A1, 0x2E39, 0x82AD, 0xCAF3, 0xE2FC, 0x94D3, 0x450F, 0xEC85, 0xD3A7, 0x31D7, 0x7057, 0x2FC4, 0xB0C4, 0x95B, 0x6DC1, 0xF7C9, 0xEF5C, 0x4F99, 0xF7D7, 0x4B14, 0xD0D6, 0xA77A, 0xCD71, 0xA1D4, 0x9BB4, 0xCF8D, 0xB898, 0x6F9E, 0xBD1B, 0x367, 0x438B, 0x6118, 0xA29, 0x558B, 0x46CA, 0xC7E3, 0xEBBF, 0x3721, 0x49C8, 0xA444, 0x303F, 0x1F26, 0xF7A3, 0x4205, 0x4033, 0x8022, 0x2DFF, 0xCD0F, 0x40D0, 0xABD1, 0x1EDD, 0x9AD4, 0x8A18, 0xD51B, 0xE8D8, 0x3291, 0x4F19, 0xC09A, 0x31ED, 0xA53D, 0xBAB9, 0xA6BD, 0xD0EC, 0x5BC8, 0x27C, 0xBF50, 0xD4C7, 0xD8EF, 0x6C2E, 0x98A4, 0x6DBF, 0xF101, 0xEBE0, 0x8960, 0xCE6E, 0x5783, 0xA55, 0x8C5F, 0x7A6B, 0xC64D, 0x3F64, 0x86B5, 0xEF84, 0x85D2, 0x9606, 0x2267, 0x89B1, 0x97A0, 0x74E1, 0xCB8C, 0xC592, 0xA3CC, 0x6EC3, 0xEFFE, 0xD2D0, 0x4F79, 0x45E1, 0x3895, 0x9E7C, 0x8471, 0x4BA, 0xDC00, 0xF327, 0x86CA, 0x3DF7, 0xBC11, 0x5C24, 0x875A, 0x4A0D, 0xD93A, 0xD437, 0x7A8A, 0x31F7, 0xDD58],
                        [0x263C, 0xF8EA, 0xA4FE, 0xE551, 0x52DF, 0xF20B, 0x89E, 0xC992, 0x6FCA, 0x7417, 0x52BC, 0x75DC, 0x3A78, 0xD7EF, 0x616, 0x9D51, 0x95DC, 0x76A5, 0x4A6D, 0xF44E, 0xC7D6, 0x6FF6, 0xE31C, 0x9D06, 0xF881, 0x1447, 0xCD8B, 0x5B10, 0xC9F5, 0x8E44, 0x326C, 0xDCB6, 0xF18C, 0xB79, 0xBBB1, 0xD008, 0xE98A, 0xA9DA, 0xFF3E, 0x62AC, 0x3CB7, 0x2A7C, 0xDD3D, 0xF774, 0xC333, 0x3041, 0x67CF, 0x1543, 0xA90A, 0x1DE4, 0xE12F, 0xB7D, 0x4359, 0x8DAD, 0xCE1A, 0x8EB7, 0x61CE, 0xB115, 0x2ACF, 0xA03E, 0xCC59, 0x2E3, 0x7365, 0x29A2, 0xCF0C, 0x884B, 0x776C, 0xC186, 0xF787, 0xBDB3, 0xDC7C, 0xA76C, 0xEC90, 0x32D0, 0xB3CE, 0x341D, 0xA18E, 0xC308, 0xF265, 0xBBC4, 0x78C3, 0x8EA8, 0xD930, 0xBBE9, 0x2339, 0xD4C, 0xCA03, 0xCB2D, 0x361D, 0xAAA4, 0x6339, 0xF908, 0x58E0, 0x8511, 0x87A5, 0xDC61, 0xC071, 0xB85D, 0xC87F, 0xB0DF, 0x704B, 0xD762, 0x30DF, 0x5876, 0xAFEE, 0xB0F3, 0x2F19, 0x7770, 0xEAB1, 0xE72D, 0x76A4, 0xCC6F, 0x1FE9, 0xFBF9, 0x57BB, 0x6822, 0x3C31, 0xE180, 0x2B7D, 0x4AA7, 0x36E0, 0x3195, 0x7D4, 0xD3B6, 0xA538, 0xB249, 0x187, 0x166C, 0x272D, 0xF121, 0x89AB, 0x9F10, 0x66CD, 0x8262, 0x5B74, 0x209A, 0xF6E4, 0x51B7, 0x875E, 0x698A, 0x64C2, 0xC899, 0x4D4C, 0xE011, 0x15E8, 0xE323, 0x50C3, 0xFBFD, 0xA904, 0x632E, 0x37E4, 0xC921, 0x3076, 0xCCEE, 0x122C, 0x48F7, 0x73CD, 0xF507, 0xE6C6, 0x79EF, 0x11AC, 0xECFD, 0x3BBC, 0x1BA8, 0x3FDA, 0xC5F2, 0x9C9D, 0xC4FE, 0x467, 0xDC6D, 0x4956, 0xE8A9, 0x707D, 0xF96D, 0x6592, 0x63DC, 0x7FEC, 0x9AE2, 0x11FC, 0x8008, 0x6665, 0x4730, 0xAC4E, 0xDD82, 0x1217, 0x3693, 0x3528, 0x8B81, 0xF5FC, 0x88E4, 0x6DD4, 0x9889, 0x2269, 0xB80D, 0x905A, 0x6C19, 0xB482, 0xDBA6, 0x79A2, 0x258B, 0x9AA5, 0x1A3C, 0x14BD, 0xC1A5, 0x45D2, 0x6C28, 0xCB25, 0x72D0, 0xF440, 0x18C1, 0x9717, 0xE76, 0xD517, 0xD2D4, 0xB63A, 0x669B, 0x7599, 0x8573, 0xB910, 0x8761, 0x71B7, 0xD5AA, 0x23D0, 0x7A11, 0x5883, 0xBCD9, 0x5025, 0xA02A, 0x1CA3, 0x72B7, 0x91E2, 0x60E9, 0x1C7, 0xAD66, 0xC80F, 0x9176, 0xFCA9, 0x2151, 0xE4A9, 0x70FC, 0x991C, 0xA16B, 0x834F, 0x483B, 0x959F, 0x7216, 0xFC2F, 0x2707, 0x46E0, 0x771A, 0xE168, 0xEF3, 0xEFCC, 0x3D3C, 0x257D, 0x8DCA],
                        [0xE309, 0xF42B, 0xE2DC, 0xDB4C, 0x7BDD, 0x42A2, 0xB82B, 0xC198, 0x2747, 0x3E7A, 0x7A3E, 0x46FF, 0x255B, 0x9EA7, 0x5D5F, 0x178A, 0xFAAD, 0x89F, 0x9632, 0x4EC0, 0x248F, 0xE81D, 0xF752, 0x6835, 0x565D, 0xF6EC, 0xAFB0, 0xC1D, 0xF0B5, 0x2D0D, 0xC11F, 0xDF30, 0xED61, 0x36B1, 0xEBF5, 0xD303, 0x5AC8, 0x20D0, 0xBEA7, 0x8D3E, 0x36F6, 0x63D1, 0xC58A, 0x15C, 0x5B45, 0xC4A6, 0x7731, 0x1712, 0xB581, 0x94F6, 0xB2E0, 0x11B, 0x6599, 0xAEA4, 0xD84, 0x3E2B, 0xD0FC, 0x6D5C, 0x987D, 0xDAE2, 0x7C7B, 0xBD85, 0x9E3A, 0xDEC6, 0x9B9A, 0x5566, 0xD802, 0x1D25, 0x3B31, 0x356C, 0x8F4E, 0x63D7, 0x82A4, 0x55BA, 0xD988, 0x31EC, 0xC275, 0xD0FA, 0xD009, 0xBCF6, 0xBE36, 0x4D3B, 0x4AD0, 0x839, 0xCBF8, 0x7849, 0xF16B, 0x73F0, 0xD807, 0x2D5E, 0xE531, 0xDE3B, 0x6EC6, 0x23C0, 0x5A61, 0x62A, 0x95D3, 0x97AD, 0xB016, 0xB85A, 0x326D, 0x32B7, 0x7E82, 0x3E85, 0x5D2E, 0xC210, 0x85C1, 0xFE90, 0xF832, 0xEF4A, 0xB9A4, 0xC52C, 0xF044, 0xF302, 0x970, 0xA7B3, 0x472D, 0xAB41, 0x22A5, 0xBE4C, 0xACA8, 0xCFB4, 0x7428, 0xD63C, 0xA499, 0x5195, 0xFB1D, 0xCA9F, 0xD98C, 0x7604, 0x7DDD, 0x47A, 0x1482, 0xE7D1, 0xC737, 0xF4B0, 0xFBC, 0x8504, 0x51F7, 0xE39A, 0x71DD, 0xC51A, 0x81C0, 0x8809, 0xAF2F, 0xBF4C, 0x73F4, 0x4438, 0xFC99, 0x90DA, 0xB425, 0x4823, 0x4D13, 0x85C0, 0x6214, 0x5A01, 0xE79D, 0xB033, 0x5AF4, 0x3A17, 0xBAD1, 0xAB6F, 0xE592, 0xDE0B, 0x68FE, 0x713C, 0xCB15, 0xE01A, 0x568, 0xE241, 0x433D, 0x4B77, 0x668, 0xECF2, 0xD751, 0xDB21, 0x456D, 0xE80C, 0x62C6, 0x2D9D, 0xF1CD, 0xBC29, 0x7E49, 0x1E13, 0xCD9, 0xECC2, 0x53D2, 0xAC1D, 0x9B17, 0x7C58, 0x55B7, 0x48E5, 0x8FB0, 0x8841, 0xC0B1, 0x5D5E, 0x1D9D, 0x16EA, 0xC267, 0x87EE, 0x7E17, 0xB83A, 0xE216, 0xB9E2, 0xB230, 0x27C1, 0x2F6, 0x1479, 0x2FB5, 0x525D, 0x4CDE, 0x707C, 0x86FA, 0xBCF1, 0xDAAC, 0x85B9, 0xFE8A, 0x5E9A, 0x8CF9, 0x2DBB, 0xC17C, 0x6D61, 0xAE4C, 0xF1F2, 0xC2BF, 0x1ECC, 0x77CD, 0xFB51, 0xCC2, 0x9482, 0x4AFB, 0x9800, 0x2490, 0xEBE2, 0xD1A0, 0xD2EC, 0x697, 0xE6B, 0xC9D4, 0x9E2, 0x101F, 0xB62, 0x5027, 0xDD9D, 0x7CC1, 0xC4D0, 0x4130, 0xC471, 0x511B, 0x87F, 0x5ABF, 0x107E, 0x38E6, 0xE1E2, 0x4B52, 0xD211]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 32]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u16, rrr.as_mut_ptr() as *mut u16, 256 * 3); }
    
    print!("RSA_4096_u16: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(rrr, message);

    // Example for RSA_1024_u8
    use cryptocol::asymmetric::RSA_1024_u8;

    let public_key = U1024::from(7_u8);
    let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    let modulus = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    let rsa = RSA_1024_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    println!("RSA_1024_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_1024_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_1024_u8: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u8; 128]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u16, m.as_mut_ptr() as *mut u16, 64 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_1024_u8: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x1C, 0xEC, 0x81, 0xE1, 0x7E, 0x3D, 0xC5, 0x86, 0x4B, 0xE8, 0x1E, 0xC, 0x8F, 0xE2, 0x4E, 0xC6, 0x2C, 0x27, 0xB1, 0xBC, 0x8D, 0x7F, 0xCF, 0xDC, 0x7E, 0xE5, 0xCE, 0x5C, 0x6, 0xF8, 0xE7, 0xDE, 0xE3, 0x66, 0xE, 0x26, 0xB4, 0x6F, 0xF6, 0xEA, 0x28, 0x9D, 0xC2, 0x84, 0x75, 0x1A, 0x33, 0xF3, 0x8E, 0xF5, 0x98, 0x6A, 0x9B, 0x30, 0x51, 0xAF, 0x2A, 0x19, 0x8A, 0x98, 0x3D, 0x6E, 0xD1, 0x36, 0x25, 0x94, 0x7D, 0x8F, 0x3B, 0x28, 0x25, 0xD1, 0x13, 0xAC, 0xA9, 0x64, 0x35, 0x35, 0x29, 0xBA, 0xF6, 0x37, 0xA9, 0x48, 0x32, 0x39, 0x7A, 0x34, 0xA9, 0x3A, 0x8E, 0x67, 0xE5, 0x50, 0x6C, 0x59, 0xF3, 0x32, 0x2F, 0x37, 0xA, 0xDF, 0xCA, 0x92, 0xE0, 0x41, 0x3A, 0x5, 0x75, 0xDD, 0x7C, 0x52, 0x79, 0x46, 0x7C, 0x63, 0x1C, 0x5D, 0x57, 0x16, 0x3, 0x27, 0x45, 0x1E, 0xFA, 0xAA, 0xA8, 0xC2],
                        [0x27, 0x1C, 0x97, 0x6E, 0x9E, 0x9F, 0x28, 0x9D, 0xE1, 0xCB, 0x90, 0x58, 0xD0, 0x4, 0xB6, 0xDC, 0xE4, 0x2, 0xFF, 0x3D, 0xDF, 0xE, 0xB8, 0x8B, 0x71, 0x59, 0x2, 0x16, 0xFB, 0x3D, 0xF8, 0x47, 0xED, 0x89, 0xDB, 0x15, 0x1E, 0xD8, 0xD3, 0x37, 0x12, 0x40, 0x78, 0x1C, 0xB2, 0xA8, 0xB5, 0x96, 0xEB, 0xCD, 0x73, 0x9B, 0x1C, 0xEB, 0xC3, 0x19, 0x9B, 0xAF, 0x80, 0x73, 0x11, 0x5B, 0xC0, 0xC7, 0xB1, 0xFF, 0x78, 0x87, 0x51, 0x4, 0x98, 0xA4, 0xE, 0xEF, 0x17, 0x78, 0xB0, 0x9B, 0xEC, 0xFB, 0xC3, 0x26, 0x52, 0x77, 0x7F, 0x9B, 0xDF, 0x17, 0x76, 0x68, 0xD8, 0xBA, 0xD8, 0xC2, 0x82, 0xD4, 0xCB, 0x4C, 0xD5, 0x3A, 0x15, 0x6A, 0x20, 0x63, 0x8F, 0x23, 0x80, 0x53, 0xDA, 0x3C, 0x89, 0xCF, 0x53, 0xFB, 0x18, 0xD9, 0x36, 0xCC, 0x78, 0x2, 0x4F, 0x6E, 0x96, 0x88, 0x8A, 0x1A, 0x9F, 0xED],
                        [0x30, 0x2B, 0x2F, 0x73, 0x12, 0xB3, 0xF2, 0x4F, 0xFE, 0xC5, 0x4B, 0x76, 0xC2, 0xAA, 0x83, 0x1A, 0x7A, 0xE9, 0x97, 0x6B, 0x98, 0xA2, 0x40, 0x87, 0xCD, 0x1C, 0xF, 0xD, 0x8D, 0x48, 0xE5, 0x6B, 0xB6, 0xFA, 0xBC, 0x55, 0x33, 0xDD, 0xD8, 0x44, 0xE3, 0xF5, 0xBD, 0xF0, 0xF, 0xA2, 0xF2, 0xA4, 0x23, 0x47, 0xD6, 0x5E, 0xC7, 0xA2, 0xFE, 0xDF, 0x1E, 0x5E, 0xB5, 0xB, 0x21, 0x8D, 0x25, 0xBA, 0xB8, 0xD8, 0xA9, 0x75, 0xE1, 0x7F, 0x8, 0x41, 0x42, 0x31, 0x3A, 0xF7, 0x1, 0x19, 0x55, 0x97, 0xEC, 0x4F, 0x53, 0xB4, 0xF3, 0xAE, 0xEA, 0x94, 0x91, 0x7F, 0xA0, 0x34, 0x7E, 0xB1, 0x31, 0x3A, 0x97, 0xCF, 0x49, 0x6C, 0xA1, 0x50, 0x7A, 0xAB, 0xE4, 0xD8, 0xAF, 0xE, 0xCB, 0xD4, 0xDE, 0x3B, 0x6A, 0xC5, 0x60, 0x13, 0xEA, 0x3D, 0x5, 0xA8, 0x91, 0x71, 0x96, 0xF4, 0xFF, 0xC7, 0x35, 0x97]]);
    let mut rrr = [[0u128; 8]; 3];
    let recovered = rsa.decrypt_array_unit(&cipher);
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u8, rrr.as_mut_ptr() as *mut u8, 128 * 3); }
    
    print!("RSA_1024_u8: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
                    [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
                    [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    assert_eq!(rrr, message);

    // Example for RSA_2048_u8
    use cryptocol::asymmetric::RSA_2048_u8;

    let public_key = U2048::from(5_u8);
    let private_key = U2048::from_str_radix("49E61033258BFDFF87ADB347816E3D13BC07A9929F4064C5336F5FEC074C857BA6F64415DB647DD6CEE42842DF409A2B036839B814E6F2A1E507F2CDD91FB2B42BE6B4A40E0CFA0779437B42B8996CA31034CB90D84CB283870BE414756316FB300523F700BCC3D9E1B8183F8EA4675FF6F59D84FF3E120CE54C94B0A44E2C1DCCA68F3F0842DB99FCD7698234C374ABB28C2B5F5A5B1914F15F41163E16A54AEDA0CB6679EA1026BA97C00B0726DB7837F7478275D3688D2981898A7BB040CC9927F4204E8BA93B97992D1C13C17D9A53F4B834A2A56DD9633F5288BA125859413D3F197F7278F95F37503D2FDDF9F540154C491AAE943B13EF3C047AA5E1AD", 16).unwrap();
    let modulus = U2048::from_str_radix("7b2a1affe93ea7548ccc2acc826265cb8eb76ff45ec0a7f355b99fdeb6d4de78c0efc6cf18522710ae26edc4c96bab9d05adb58822d63f0dd30d3f57148a29d6f3d5d7bc176af60c74c5cd6f33aa5fba7057fdf1687fd4308bbe7c2218fa7ba2a55de69babe5466b22dd7dbf4312019ff0eeb132febcc8c028d4f7d111d79eddc056ec9cd74e3a369eefbc634b6544bcbd80ff73a941ab7614ccf740da8a1010a6e1799953abfdcbe9892520eb35733602807ef52fb6b4cf0664560e2feda618cceecba02038d0c3349e1634f778544f2ab941bd61788f30db3c0aeebefb02759f2e605f301e48b5235f3340fecb37e04bd34f53a2f516904532135a4a38b9e7", 16).unwrap();
    let rsa = RSA_2048_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                    [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                    [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]];
    println!("RSA_2048_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_2048_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_2048_u8: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u8; 256]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, m.as_mut_ptr() as *mut u8, 256 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_2048_u8: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x46, 0x27, 0x51, 0x58, 0x9, 0xE3, 0x8D, 0xA8, 0xA0, 0xFC, 0xA1, 0x16, 0xD1, 0xDA, 0x39, 0x8, 0xA1, 0x93, 0xC5, 0x63, 0xFB, 0x5B, 0x65, 0x73, 0x26, 0x1B, 0x99, 0x7, 0x64, 0x2D, 0xEF, 0xBF, 0x6F, 0x85, 0x2, 0x86, 0xD5, 0xFF, 0x4B, 0x6C, 0xDA, 0x69, 0x69, 0x4B, 0xB1, 0x1C, 0xE9, 0xFA, 0x3, 0x2, 0x34, 0xCE, 0xE8, 0x11, 0x78, 0x48, 0x2B, 0x5A, 0xFF, 0x55, 0x93, 0x44, 0x34, 0x3A, 0x17, 0x3E, 0x93, 0x40, 0x8E, 0xFC, 0xE7, 0xB6, 0xA2, 0x3C, 0xBB, 0xD1, 0xC4, 0x6C, 0xF4, 0xCB, 0xF6, 0xD9, 0x99, 0xA7, 0xB4, 0x95, 0x11, 0xA7, 0x6E, 0xED, 0x4, 0xC9, 0x80, 0x3C, 0xEE, 0xC, 0xC0, 0x13, 0x7, 0xC5, 0x59, 0x6E, 0x39, 0x7C, 0x79, 0x5, 0x2D, 0xC3, 0x92, 0x9F, 0x83, 0x16, 0x80, 0xCD, 0xAA, 0xB7, 0x10, 0x32, 0x91, 0x32, 0x12, 0x31, 0xAB, 0x66, 0xAC, 0xBE, 0x85, 0xF8, 0x1A, 0x7B, 0xBC, 0x75, 0xAF, 0x37, 0x4B, 0x8E, 0xFA, 0x5E, 0x8E, 0xDC, 0x53, 0x6E, 0x5E, 0xD5, 0x6A, 0x8D, 0x16, 0x55, 0x6F, 0x17, 0x8C, 0xAC, 0xD7, 0x29, 0xF5, 0x55, 0x8B, 0x77, 0xF6, 0x1C, 0x46, 0xC3, 0x27, 0x69, 0x0, 0xCD, 0x28, 0xB8, 0x96, 0x2E, 0x1F, 0xFF, 0x3C, 0xA7, 0x33, 0x87, 0x8B, 0xEC, 0xF3, 0xDC, 0xC1, 0x6A, 0x7A, 0x10, 0x7B, 0x31, 0xE4, 0x87, 0x7A, 0x89, 0xEA, 0x55, 0x8C, 0x9F, 0x4B, 0x48, 0x38, 0x9B, 0x10, 0x7D, 0x8C, 0xC1, 0xE3, 0x72, 0xA, 0x10, 0x52, 0x7A, 0xD9, 0x4C, 0x57, 0x69, 0x3E, 0x2D, 0x23, 0xD2, 0x47, 0xE8, 0x16, 0xEC, 0x5E, 0xF2, 0xAA, 0xA0, 0x1E, 0x25, 0x12, 0x78, 0x69, 0x75, 0x72, 0x52, 0x91, 0x16, 0x21, 0x44, 0x56, 0xC1, 0x7C, 0x1A, 0x97, 0x9D, 0x8B, 0x45, 0x41, 0x26, 0xA, 0xF7, 0x5B, 0xE4, 0x7C, 0x8D, 0xDE, 0x8C, 0x4B, 0x14],
                        [0x4A, 0xA3, 0x8E, 0xAF, 0x7A, 0xF7, 0x51, 0x84, 0xB, 0xE1, 0xC9, 0x1C, 0x28, 0x8D, 0x6F, 0x1A, 0x5B, 0x53, 0x1E, 0x1B, 0x2F, 0xF5, 0x36, 0xD6, 0xDC, 0x59, 0xB7, 0x16, 0x8B, 0x6A, 0x4, 0x0, 0x47, 0x49, 0xDB, 0xA4, 0xD4, 0x3, 0x46, 0x69, 0xD1, 0x57, 0x73, 0xD4, 0xCB, 0x44, 0xAA, 0xFA, 0xA, 0xD5, 0xA7, 0x18, 0x67, 0xF5, 0x1F, 0x28, 0xFD, 0x48, 0x16, 0xE3, 0x2, 0xF5, 0x96, 0xB9, 0x7F, 0xC5, 0xFF, 0xF6, 0xC7, 0x9, 0x15, 0x1E, 0xC6, 0xC1, 0xCB, 0xA3, 0x70, 0x59, 0xA, 0x43, 0x3, 0x1C, 0x50, 0x87, 0x4B, 0x7C, 0xDE, 0xBD, 0x83, 0x4F, 0x10, 0x6A, 0xD1, 0x86, 0xC8, 0x9A, 0xB3, 0xFC, 0xE5, 0xAF, 0x65, 0xD3, 0xE6, 0xCA, 0xB4, 0x31, 0x1B, 0x78, 0x6A, 0x9F, 0x68, 0x15, 0x31, 0x34, 0x39, 0xEC, 0x23, 0x22, 0x4, 0xAE, 0xB9, 0x1C, 0x28, 0x94, 0xCD, 0xEE, 0xE7, 0x3, 0xB9, 0xEE, 0x46, 0xED, 0x54, 0xCB, 0x58, 0xB9, 0x46, 0xD9, 0xF0, 0xF3, 0x10, 0x9B, 0xF3, 0x41, 0xAF, 0x89, 0xAA, 0xDD, 0x2E, 0x2C, 0x15, 0xC2, 0xC5, 0x18, 0xCB, 0x2C, 0xD0, 0x2D, 0xA2, 0xB6, 0xC7, 0x1D, 0xEA, 0x4F, 0x9D, 0xD8, 0xE9, 0xE4, 0xCA, 0xBC, 0x40, 0x4B, 0xC7, 0x8C, 0x73, 0xAA, 0xBD, 0x3C, 0xCF, 0x1F, 0xA8, 0x1C, 0xA4, 0x30, 0x68, 0xF1, 0x4B, 0x9C, 0x62, 0x9, 0x8C, 0xE7, 0xDE, 0x42, 0x6F, 0xA9, 0xC7, 0x30, 0xFE, 0x25, 0xC8, 0x70, 0x49, 0x4, 0x20, 0x71, 0x95, 0x4, 0x6E, 0xA2, 0xBE, 0xF4, 0xE2, 0x80, 0x88, 0xC, 0x74, 0xFB, 0xDB, 0xB6, 0xE5, 0x4C, 0xDA, 0x5D, 0xD0, 0x2C, 0xFE, 0x2A, 0x6E, 0xB, 0x86, 0xA6, 0xCA, 0x27, 0xE2, 0x72, 0xCA, 0x53, 0x1E, 0xB9, 0x72, 0x73, 0xC3, 0x7D, 0xC1, 0xBA, 0x31, 0xEC, 0xEF, 0xA0, 0xDF, 0x6, 0xC6, 0xBD, 0x4A, 0x20],
                        [0x76, 0x71, 0xC9, 0x3E, 0x31, 0x22, 0x66, 0x3E, 0x9, 0x2C, 0x14, 0xD7, 0xA7, 0xF0, 0x2D, 0x87, 0xC9, 0x4, 0x2E, 0x91, 0x27, 0xB8, 0x2E, 0x9A, 0xDF, 0x5C, 0xED, 0xE5, 0xE9, 0xBD, 0x6C, 0x4A, 0x95, 0xC1, 0xD9, 0x8A, 0x54, 0x68, 0xEC, 0x81, 0xDD, 0x5, 0x5F, 0xDD, 0x5D, 0x0, 0xD9, 0x3, 0xE4, 0x93, 0xB6, 0x3D, 0xB0, 0xA4, 0x6B, 0x40, 0xA5, 0x7B, 0x9A, 0xCC, 0x10, 0x1B, 0xF8, 0xC7, 0xA7, 0xEE, 0xF, 0x5B, 0x94, 0x5A, 0x19, 0x78, 0x4A, 0x9F, 0xCC, 0x45, 0xF6, 0x86, 0x36, 0x5C, 0x23, 0xA9, 0x7E, 0xB1, 0xF, 0xD4, 0xA9, 0xCE, 0x35, 0xC9, 0x43, 0x65, 0x69, 0xEB, 0x44, 0xEB, 0x87, 0x6F, 0x40, 0x9C, 0x10, 0x27, 0x4C, 0x46, 0x3C, 0x37, 0xEE, 0x3D, 0x32, 0xCB, 0x63, 0xED, 0xB4, 0xD1, 0x84, 0xF7, 0xBA, 0xC7, 0xE9, 0x8D, 0xC6, 0xB, 0x16, 0x3, 0x87, 0xE1, 0xA4, 0x38, 0x11, 0x94, 0xC4, 0xD7, 0x78, 0x60, 0x48, 0xF8, 0x0, 0xB, 0x3, 0x40, 0xAD, 0x7F, 0x4, 0x97, 0x9F, 0xA8, 0x37, 0xB8, 0xF5, 0x34, 0xE, 0xB0, 0x66, 0x9A, 0x27, 0x16, 0xED, 0x4F, 0x8F, 0xD6, 0x97, 0xD8, 0xB, 0x47, 0xD0, 0xAE, 0xD4, 0x5E, 0xC2, 0x60, 0xB3, 0x2B, 0x2C, 0x88, 0x1A, 0x8E, 0x94, 0x3D, 0xA7, 0x74, 0xF4, 0x57, 0x12, 0xDC, 0xA2, 0xFB, 0x8D, 0x5B, 0xE5, 0x37, 0xE7, 0x64, 0x52, 0x81, 0x5C, 0x39, 0x68, 0x2C, 0x82, 0xC7, 0xB4, 0xD0, 0x9A, 0x7C, 0xCD, 0x4C, 0x13, 0x69, 0xD6, 0x35, 0x25, 0xBD, 0xF6, 0xED, 0x72, 0x4F, 0xA4, 0xF3, 0x1, 0x11, 0x4C, 0xB, 0xA6, 0x4A, 0x35, 0xEC, 0x3, 0xBB, 0xE7, 0xD1, 0x86, 0xF8, 0xDF, 0xFA, 0x85, 0xC, 0xFB, 0x17, 0x16, 0xE1, 0x6, 0x5E, 0x0, 0xAA, 0x2F, 0x99, 0xD3, 0xBA, 0xBB, 0x84, 0xBD, 0x90, 0xCD, 0x50, 0xAA, 0x10]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 16]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u8, rrr.as_mut_ptr() as *mut u8, 256 * 3); }
    
    print!("RSA_2048_u8: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF],
                        [0x_21123456789ABCDEF00FEDCBA9876543_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_11112222777788883333444455556666, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_11223344556677889900AABBCCDDEEFF, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_0F1E2D3C4B5A6978879605A4B3C2D1E0, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_0000AAAABBBB11112222333344449999, 0x_CCCCDDDDEEEEFFFF5555666677778888],
                        [0x_321123456789ABCDEF00FEDCBA987654_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_1F2E3D4CC4D3E2F15B6A70899807A6B5, 0x_F1E2D3C4B4C3D2E1F5A6079889706A5B, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_FEDCBA989ABCDEF87654321001234567, 0x_CCDDEEFF112233449900AABB55667788, 0x_FFEEDDCCBBAA00998877665544332211, 0x_9807A6B51F2E3D4C5B6A7089C4D3E2F1, 0x_879605A44B5A6978B3C2D1E00F1E2D3C, 0x_5D4E3F201102F3E98A7B6C4D5C6B7A89, 0x_11112222AAAABBBB3333444499990000, 0x_77778888CCCCDDDD55556666EEEEFFFF]]);
    assert_eq!(rrr, message);

    // Example for RSA_4096_u8
    use cryptocol::asymmetric::RSA_4096_u8;

    let public_key = U4096::from(5_u8);
    let private_key = U4096::from_str_radix("4E0F13FFD94F4F793ED6900C7A99EA88942D3A065EB7E72E741A77CE7BB6AD5B7FFB2B83B6154A975FAE613F22A739F54E305BDCD85A5A3D5C59C28FEDB6AC2CF03272A57C25ED4CEAAF8006321DA2529F381EF51B050FA1AC7DE541624A19AFE8561C1407F9D3333AA364DDA03A5E50ACA61C7E52850C519549AB45B82AE1E831D7FE401A641CB47A4711340F4B6D2F9822D32442168D2517B5CB9E6A01FB89D9B8240858269DA12F8B0E247AD1D0DA835C3F38390EB4BA56EAA266FF113CA7DD7817AF156E994C21738839D1E4B6AE4B04332F09DC6A047F75FF521E6868697998CBCE9CBA7C505A97C01C510BED5CDBB55D4E624CA14A9D5A4A05851F0842904604D544B6FBF154BAF9FAB89A2765A41A6BD17E631B079429AED230B2F72777350CEF5CA6E89784376003BA8F189F671D9451EE86039EF8F2DE7B6E9A3A37A4C15BD8ACE6CCA3B218CD59D018674DDE5BCF25487464AFDA648A475D6BCEA1A95F2E2937DB1CEE3C9E7EBB2492ED275DCD9089A2909CBBDEB0368EDC5E6DBD12582E171DF0724C5534204F656E095339C5E19D5665D214D3E4F05F6A18F3D0B3E95D16996EE3E49A239C8673A78D0DD90F36DBE6F85B316DD1E2AFED84D3823F5EAE9088176309CC33056B2976C31C8EC94C5FAA62E466CFC8FFE8D6DCB106DB27BA551C1A041561E597C967C1C8568D3C605BD703877343E0C4B4E932E10D", 16).unwrap();
    let modulus = U4096::from_str_radix("6192d8ffcfa323578e8c340f9940652ab9388887f665e0fa112115c21aa458b25ff9f664a39a9d3d3799f98eeb510872a1bc72d40e70f0ccb3703333e92457382c3f0f4edb2f68a0255b6007bea50ae7470626b261c6538a179d5e91badca01be26ba31909f84800094c3e150848f5e4d7cfa39de7264f65fa9c161726359a623e4dfdd020fd23e198d8d581131e487b7e2b87ed529c306e5da33e8604827a6c50262d0a6e3045097b6dd1ad9986451124334f06475261e8eca54b00bed58bd1d4d61d9adaca3f9f29d06a48465de459ddc53ffacc5384859f537f26a6028283d7fefec243e91b64713db023654ee8b412a2b4a1fadfc99d44b0dc86e666ca5470704b8bfe5fd708ddd576bd021bf4c2d813279ec0d74f4e03a53557f2cefde2cd9e9d01c72be14754f7a4472dfe8ef653fe73980d1edc3727501f5594863d2e32a8abf8fc04c44f8aac2e9bda32a8d154a62107bdc1f2f394b9ae693e84d6c7ba4143c15b39fed115fd4265fc230c3e9cd830d56f7866e09042569c80517596f8fcef6ed1e31962d3805d2d811248b11006e1dc14d03c98f5daeca854ad44eac60878e807d26656e5f91e4cfeeb16b8f8ba4d04d398737af8fd4a6b7f1b31e390dee1dfc5e117cdd76b4e26b2407480d97a1503f8aa01f676d6ddbce264e1cd4ec3f2ef903632865fe85795d5298432df45cae39771f83155b8da0979de9f23", 16).unwrap();
    let rsa = RSA_4096_u8::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulus.into_biguint());
    let message = [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]];

    println!("RSA_4096_u8: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulus());
    println!("RSA_4096_u8: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulus());
    print!("RSA_4096_u8: Message = [");
    for mm in message
    {
        print!(" [");
        for m in mm
            { print!("{:#X}, ", m); }
        print!(" ], ");
    }
    println!("]");

    let mut m = [[0u8; 512]; 3];
    unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, m.as_mut_ptr() as *mut u8, 512 * 3); }
    let cipher = rsa.encrypt_array_unit(&m);
    print!("RSA_4096_u8: Cipher = [");
    for cc in cipher
    {
        print!("[");
        for c in cc
            { print!("{:#X}, ", c); }
        print!("], ");
    }
    println!("]");
    assert_eq!(cipher, [[0x1C, 0x44, 0x7D, 0x29, 0x11, 0x64, 0x5F, 0x2C, 0xEE, 0x7F, 0x3A, 0x75, 0x2C, 0x11, 0xC6, 0x50, 0x5A, 0xF7, 0x65, 0x97, 0x73, 0x7B, 0x96, 0xC, 0xEB, 0x45, 0x4B, 0x46, 0xE2, 0x2C, 0x7B, 0x2E, 0x27, 0xD1, 0xD8, 0xA, 0x21, 0x4F, 0x32, 0xBE, 0x7D, 0x22, 0x8D, 0x71, 0x8B, 0x4C, 0x64, 0xAA, 0xAD, 0x30, 0x85, 0x47, 0x2D, 0x80, 0x60, 0xB0, 0x9, 0x84, 0x68, 0x5E, 0x94, 0xC8, 0xF3, 0x82, 0x71, 0xDB, 0x30, 0x99, 0x36, 0x7A, 0x28, 0xA2, 0x6F, 0x76, 0xE5, 0xE9, 0x5C, 0x5C, 0x9E, 0xC0, 0x1E, 0xBA, 0xDE, 0xB4, 0x9E, 0x2F, 0x45, 0xDD, 0x95, 0x52, 0x19, 0x5D, 0x41, 0xD2, 0x66, 0xA3, 0x2E, 0x15, 0xE, 0x5C, 0xC6, 0x47, 0xAB, 0xED, 0xE, 0x7, 0xDE, 0x6B, 0x6F, 0xAA, 0xA5, 0x27, 0xFB, 0x9, 0x40, 0x13, 0x8D, 0x3, 0xB1, 0x55, 0x46, 0x64, 0x19, 0x19, 0x64, 0xC2, 0xA9, 0xC4, 0x51, 0xAB, 0xC6, 0xAE, 0xD5, 0xDC, 0x37, 0xA4, 0x60, 0xC5, 0xEB, 0xF8, 0xFC, 0xB1, 0xAD, 0x9E, 0x23, 0xD4, 0x46, 0xA4, 0x32, 0xC6, 0x10, 0x6C, 0x26, 0x83, 0x55, 0x20, 0xBC, 0xD5, 0x92, 0xEF, 0xC2, 0x75, 0x4F, 0x12, 0x15, 0x65, 0x31, 0x61, 0x6D, 0xA0, 0xA2, 0x9B, 0xA3, 0x39, 0x2, 0xC4, 0xE8, 0x40, 0x64, 0x82, 0x8B, 0xED, 0x4, 0xFF, 0x8F, 0xA3, 0xE3, 0x3B, 0xA7, 0xED, 0xFB, 0x8F, 0x25, 0x4B, 0x15, 0xBB, 0xD3, 0x69, 0x7A, 0xF3, 0x50, 0x1E, 0xA2, 0x82, 0x5E, 0x86, 0x99, 0xE7, 0x91, 0x1E, 0xFC, 0xB9, 0xF8, 0x3F, 0xDA, 0x8D, 0xB5, 0x73, 0x26, 0xA9, 0xDF, 0x7B, 0x6B, 0x2E, 0xAB, 0x30, 0x66, 0x2C, 0xAF, 0xFF, 0xF, 0xF1, 0x53, 0x62, 0xEC, 0xD1, 0x26, 0x7D, 0x7B, 0xEE, 0x27, 0xBC, 0x67, 0xE3, 0x5A, 0x23, 0x33, 0x62, 0xB5, 0x54, 0x19, 0x82, 0xA2, 0xFF, 0x16, 0x75, 0xE5, 0x54, 0xBF, 0x1B, 0x1E, 0x90, 0xA7, 0xB9, 0x59, 0x20, 0x67, 0xC6, 0x30, 0x49, 0xF, 0xE7, 0x54, 0x47, 0xA6, 0x39, 0x43, 0x22, 0xC1, 0xD4, 0xB8, 0xEB, 0xCD, 0x49, 0xA1, 0xE8, 0x39, 0x2E, 0xAD, 0x82, 0xF3, 0xCA, 0xFC, 0xE2, 0xD3, 0x94, 0xF, 0x45, 0x85, 0xEC, 0xA7, 0xD3, 0xD7, 0x31, 0x57, 0x70, 0xC4, 0x2F, 0xC4, 0xB0, 0x5B, 0x9, 0xC1, 0x6D, 0xC9, 0xF7, 0x5C, 0xEF, 0x99, 0x4F, 0xD7, 0xF7, 0x14, 0x4B, 0xD6, 0xD0, 0x7A, 0xA7, 0x71, 0xCD, 0xD4, 0xA1, 0xB4, 0x9B, 0x8D, 0xCF, 0x98, 0xB8, 0x9E, 0x6F, 0x1B, 0xBD, 0x67, 0x3, 0x8B, 0x43, 0x18, 0x61, 0x29, 0xA, 0x8B, 0x55, 0xCA, 0x46, 0xE3, 0xC7, 0xBF, 0xEB, 0x21, 0x37, 0xC8, 0x49, 0x44, 0xA4, 0x3F, 0x30, 0x26, 0x1F, 0xA3, 0xF7, 0x5, 0x42, 0x33, 0x40, 0x22, 0x80, 0xFF, 0x2D, 0xF, 0xCD, 0xD0, 0x40, 0xD1, 0xAB, 0xDD, 0x1E, 0xD4, 0x9A, 0x18, 0x8A, 0x1B, 0xD5, 0xD8, 0xE8, 0x91, 0x32, 0x19, 0x4F, 0x9A, 0xC0, 0xED, 0x31, 0x3D, 0xA5, 0xB9, 0xBA, 0xBD, 0xA6, 0xEC, 0xD0, 0xC8, 0x5B, 0x7C, 0x2, 0x50, 0xBF, 0xC7, 0xD4, 0xEF, 0xD8, 0x2E, 0x6C, 0xA4, 0x98, 0xBF, 0x6D, 0x1, 0xF1, 0xE0, 0xEB, 0x60, 0x89, 0x6E, 0xCE, 0x83, 0x57, 0x55, 0xA, 0x5F, 0x8C, 0x6B, 0x7A, 0x4D, 0xC6, 0x64, 0x3F, 0xB5, 0x86, 0x84, 0xEF, 0xD2, 0x85, 0x6, 0x96, 0x67, 0x22, 0xB1, 0x89, 0xA0, 0x97, 0xE1, 0x74, 0x8C, 0xCB, 0x92, 0xC5, 0xCC, 0xA3, 0xC3, 0x6E, 0xFE, 0xEF, 0xD0, 0xD2, 0x79, 0x4F, 0xE1, 0x45, 0x95, 0x38, 0x7C, 0x9E, 0x71, 0x84, 0xBA, 0x4, 0x0, 0xDC, 0x27, 0xF3, 0xCA, 0x86, 0xF7, 0x3D, 0x11, 0xBC, 0x24, 0x5C, 0x5A, 0x87, 0xD, 0x4A, 0x3A, 0xD9, 0x37, 0xD4, 0x8A, 0x7A, 0xF7, 0x31, 0x58, 0xDD],
                        [0x3C, 0x26, 0xEA, 0xF8, 0xFE, 0xA4, 0x51, 0xE5, 0xDF, 0x52, 0xB, 0xF2, 0x9E, 0x8, 0x92, 0xC9, 0xCA, 0x6F, 0x17, 0x74, 0xBC, 0x52, 0xDC, 0x75, 0x78, 0x3A, 0xEF, 0xD7, 0x16, 0x6, 0x51, 0x9D, 0xDC, 0x95, 0xA5, 0x76, 0x6D, 0x4A, 0x4E, 0xF4, 0xD6, 0xC7, 0xF6, 0x6F, 0x1C, 0xE3, 0x6, 0x9D, 0x81, 0xF8, 0x47, 0x14, 0x8B, 0xCD, 0x10, 0x5B, 0xF5, 0xC9, 0x44, 0x8E, 0x6C, 0x32, 0xB6, 0xDC, 0x8C, 0xF1, 0x79, 0xB, 0xB1, 0xBB, 0x8, 0xD0, 0x8A, 0xE9, 0xDA, 0xA9, 0x3E, 0xFF, 0xAC, 0x62, 0xB7, 0x3C, 0x7C, 0x2A, 0x3D, 0xDD, 0x74, 0xF7, 0x33, 0xC3, 0x41, 0x30, 0xCF, 0x67, 0x43, 0x15, 0xA, 0xA9, 0xE4, 0x1D, 0x2F, 0xE1, 0x7D, 0xB, 0x59, 0x43, 0xAD, 0x8D, 0x1A, 0xCE, 0xB7, 0x8E, 0xCE, 0x61, 0x15, 0xB1, 0xCF, 0x2A, 0x3E, 0xA0, 0x59, 0xCC, 0xE3, 0x2, 0x65, 0x73, 0xA2, 0x29, 0xC, 0xCF, 0x4B, 0x88, 0x6C, 0x77, 0x86, 0xC1, 0x87, 0xF7, 0xB3, 0xBD, 0x7C, 0xDC, 0x6C, 0xA7, 0x90, 0xEC, 0xD0, 0x32, 0xCE, 0xB3, 0x1D, 0x34, 0x8E, 0xA1, 0x8, 0xC3, 0x65, 0xF2, 0xC4, 0xBB, 0xC3, 0x78, 0xA8, 0x8E, 0x30, 0xD9, 0xE9, 0xBB, 0x39, 0x23, 0x4C, 0xD, 0x3, 0xCA, 0x2D, 0xCB, 0x1D, 0x36, 0xA4, 0xAA, 0x39, 0x63, 0x8, 0xF9, 0xE0, 0x58, 0x11, 0x85, 0xA5, 0x87, 0x61, 0xDC, 0x71, 0xC0, 0x5D, 0xB8, 0x7F, 0xC8, 0xDF, 0xB0, 0x4B, 0x70, 0x62, 0xD7, 0xDF, 0x30, 0x76, 0x58, 0xEE, 0xAF, 0xF3, 0xB0, 0x19, 0x2F, 0x70, 0x77, 0xB1, 0xEA, 0x2D, 0xE7, 0xA4, 0x76, 0x6F, 0xCC, 0xE9, 0x1F, 0xF9, 0xFB, 0xBB, 0x57, 0x22, 0x68, 0x31, 0x3C, 0x80, 0xE1, 0x7D, 0x2B, 0xA7, 0x4A, 0xE0, 0x36, 0x95, 0x31, 0xD4, 0x7, 0xB6, 0xD3, 0x38, 0xA5, 0x49, 0xB2, 0x87, 0x1, 0x6C, 0x16, 0x2D, 0x27, 0x21, 0xF1, 0xAB, 0x89, 0x10, 0x9F, 0xCD, 0x66, 0x62, 0x82, 0x74, 0x5B, 0x9A, 0x20, 0xE4, 0xF6, 0xB7, 0x51, 0x5E, 0x87, 0x8A, 0x69, 0xC2, 0x64, 0x99, 0xC8, 0x4C, 0x4D, 0x11, 0xE0, 0xE8, 0x15, 0x23, 0xE3, 0xC3, 0x50, 0xFD, 0xFB, 0x4, 0xA9, 0x2E, 0x63, 0xE4, 0x37, 0x21, 0xC9, 0x76, 0x30, 0xEE, 0xCC, 0x2C, 0x12, 0xF7, 0x48, 0xCD, 0x73, 0x7, 0xF5, 0xC6, 0xE6, 0xEF, 0x79, 0xAC, 0x11, 0xFD, 0xEC, 0xBC, 0x3B, 0xA8, 0x1B, 0xDA, 0x3F, 0xF2, 0xC5, 0x9D, 0x9C, 0xFE, 0xC4, 0x67, 0x4, 0x6D, 0xDC, 0x56, 0x49, 0xA9, 0xE8, 0x7D, 0x70, 0x6D, 0xF9, 0x92, 0x65, 0xDC, 0x63, 0xEC, 0x7F, 0xE2, 0x9A, 0xFC, 0x11, 0x8, 0x80, 0x65, 0x66, 0x30, 0x47, 0x4E, 0xAC, 0x82, 0xDD, 0x17, 0x12, 0x93, 0x36, 0x28, 0x35, 0x81, 0x8B, 0xFC, 0xF5, 0xE4, 0x88, 0xD4, 0x6D, 0x89, 0x98, 0x69, 0x22, 0xD, 0xB8, 0x5A, 0x90, 0x19, 0x6C, 0x82, 0xB4, 0xA6, 0xDB, 0xA2, 0x79, 0x8B, 0x25, 0xA5, 0x9A, 0x3C, 0x1A, 0xBD, 0x14, 0xA5, 0xC1, 0xD2, 0x45, 0x28, 0x6C, 0x25, 0xCB, 0xD0, 0x72, 0x40, 0xF4, 0xC1, 0x18, 0x17, 0x97, 0x76, 0xE, 0x17, 0xD5, 0xD4, 0xD2, 0x3A, 0xB6, 0x9B, 0x66, 0x99, 0x75, 0x73, 0x85, 0x10, 0xB9, 0x61, 0x87, 0xB7, 0x71, 0xAA, 0xD5, 0xD0, 0x23, 0x11, 0x7A, 0x83, 0x58, 0xD9, 0xBC, 0x25, 0x50, 0x2A, 0xA0, 0xA3, 0x1C, 0xB7, 0x72, 0xE2, 0x91, 0xE9, 0x60, 0xC7, 0x1, 0x66, 0xAD, 0xF, 0xC8, 0x76, 0x91, 0xA9, 0xFC, 0x51, 0x21, 0xA9, 0xE4, 0xFC, 0x70, 0x1C, 0x99, 0x6B, 0xA1, 0x4F, 0x83, 0x3B, 0x48, 0x9F, 0x95, 0x16, 0x72, 0x2F, 0xFC, 0x7, 0x27, 0xE0, 0x46, 0x1A, 0x77, 0x68, 0xE1, 0xF3, 0xE, 0xCC, 0xEF, 0x3C, 0x3D, 0x7D, 0x25, 0xCA, 0x8D],
                        [0x9, 0xE3, 0x2B, 0xF4, 0xDC, 0xE2, 0x4C, 0xDB, 0xDD, 0x7B, 0xA2, 0x42, 0x2B, 0xB8, 0x98, 0xC1, 0x47, 0x27, 0x7A, 0x3E, 0x3E, 0x7A, 0xFF, 0x46, 0x5B, 0x25, 0xA7, 0x9E, 0x5F, 0x5D, 0x8A, 0x17, 0xAD, 0xFA, 0x9F, 0x8, 0x32, 0x96, 0xC0, 0x4E, 0x8F, 0x24, 0x1D, 0xE8, 0x52, 0xF7, 0x35, 0x68, 0x5D, 0x56, 0xEC, 0xF6, 0xB0, 0xAF, 0x1D, 0xC, 0xB5, 0xF0, 0xD, 0x2D, 0x1F, 0xC1, 0x30, 0xDF, 0x61, 0xED, 0xB1, 0x36, 0xF5, 0xEB, 0x3, 0xD3, 0xC8, 0x5A, 0xD0, 0x20, 0xA7, 0xBE, 0x3E, 0x8D, 0xF6, 0x36, 0xD1, 0x63, 0x8A, 0xC5, 0x5C, 0x1, 0x45, 0x5B, 0xA6, 0xC4, 0x31, 0x77, 0x12, 0x17, 0x81, 0xB5, 0xF6, 0x94, 0xE0, 0xB2, 0x1B, 0x1, 0x99, 0x65, 0xA4, 0xAE, 0x84, 0xD, 0x2B, 0x3E, 0xFC, 0xD0, 0x5C, 0x6D, 0x7D, 0x98, 0xE2, 0xDA, 0x7B, 0x7C, 0x85, 0xBD, 0x3A, 0x9E, 0xC6, 0xDE, 0x9A, 0x9B, 0x66, 0x55, 0x2, 0xD8, 0x25, 0x1D, 0x31, 0x3B, 0x6C, 0x35, 0x4E, 0x8F, 0xD7, 0x63, 0xA4, 0x82, 0xBA, 0x55, 0x88, 0xD9, 0xEC, 0x31, 0x75, 0xC2, 0xFA, 0xD0, 0x9, 0xD0, 0xF6, 0xBC, 0x36, 0xBE, 0x3B, 0x4D, 0xD0, 0x4A, 0x39, 0x8, 0xF8, 0xCB, 0x49, 0x78, 0x6B, 0xF1, 0xF0, 0x73, 0x7, 0xD8, 0x5E, 0x2D, 0x31, 0xE5, 0x3B, 0xDE, 0xC6, 0x6E, 0xC0, 0x23, 0x61, 0x5A, 0x2A, 0x6, 0xD3, 0x95, 0xAD, 0x97, 0x16, 0xB0, 0x5A, 0xB8, 0x6D, 0x32, 0xB7, 0x32, 0x82, 0x7E, 0x85, 0x3E, 0x2E, 0x5D, 0x10, 0xC2, 0xC1, 0x85, 0x90, 0xFE, 0x32, 0xF8, 0x4A, 0xEF, 0xA4, 0xB9, 0x2C, 0xC5, 0x44, 0xF0, 0x2, 0xF3, 0x70, 0x9, 0xB3, 0xA7, 0x2D, 0x47, 0x41, 0xAB, 0xA5, 0x22, 0x4C, 0xBE, 0xA8, 0xAC, 0xB4, 0xCF, 0x28, 0x74, 0x3C, 0xD6, 0x99, 0xA4, 0x95, 0x51, 0x1D, 0xFB, 0x9F, 0xCA, 0x8C, 0xD9, 0x4, 0x76, 0xDD, 0x7D, 0x7A, 0x4, 0x82, 0x14, 0xD1, 0xE7, 0x37, 0xC7, 0xB0, 0xF4, 0xBC, 0xF, 0x4, 0x85, 0xF7, 0x51, 0x9A, 0xE3, 0xDD, 0x71, 0x1A, 0xC5, 0xC0, 0x81, 0x9, 0x88, 0x2F, 0xAF, 0x4C, 0xBF, 0xF4, 0x73, 0x38, 0x44, 0x99, 0xFC, 0xDA, 0x90, 0x25, 0xB4, 0x23, 0x48, 0x13, 0x4D, 0xC0, 0x85, 0x14, 0x62, 0x1, 0x5A, 0x9D, 0xE7, 0x33, 0xB0, 0xF4, 0x5A, 0x17, 0x3A, 0xD1, 0xBA, 0x6F, 0xAB, 0x92, 0xE5, 0xB, 0xDE, 0xFE, 0x68, 0x3C, 0x71, 0x15, 0xCB, 0x1A, 0xE0, 0x68, 0x5, 0x41, 0xE2, 0x3D, 0x43, 0x77, 0x4B, 0x68, 0x6, 0xF2, 0xEC, 0x51, 0xD7, 0x21, 0xDB, 0x6D, 0x45, 0xC, 0xE8, 0xC6, 0x62, 0x9D, 0x2D, 0xCD, 0xF1, 0x29, 0xBC, 0x49, 0x7E, 0x13, 0x1E, 0xD9, 0xC, 0xC2, 0xEC, 0xD2, 0x53, 0x1D, 0xAC, 0x17, 0x9B, 0x58, 0x7C, 0xB7, 0x55, 0xE5, 0x48, 0xB0, 0x8F, 0x41, 0x88, 0xB1, 0xC0, 0x5E, 0x5D, 0x9D, 0x1D, 0xEA, 0x16, 0x67, 0xC2, 0xEE, 0x87, 0x17, 0x7E, 0x3A, 0xB8, 0x16, 0xE2, 0xE2, 0xB9, 0x30, 0xB2, 0xC1, 0x27, 0xF6, 0x2, 0x79, 0x14, 0xB5, 0x2F, 0x5D, 0x52, 0xDE, 0x4C, 0x7C, 0x70, 0xFA, 0x86, 0xF1, 0xBC, 0xAC, 0xDA, 0xB9, 0x85, 0x8A, 0xFE, 0x9A, 0x5E, 0xF9, 0x8C, 0xBB, 0x2D, 0x7C, 0xC1, 0x61, 0x6D, 0x4C, 0xAE, 0xF2, 0xF1, 0xBF, 0xC2, 0xCC, 0x1E, 0xCD, 0x77, 0x51, 0xFB, 0xC2, 0xC, 0x82, 0x94, 0xFB, 0x4A, 0x0, 0x98, 0x90, 0x24, 0xE2, 0xEB, 0xA0, 0xD1, 0xEC, 0xD2, 0x97, 0x6, 0x6B, 0xE, 0xD4, 0xC9, 0xE2, 0x9, 0x1F, 0x10, 0x62, 0xB, 0x27, 0x50, 0x9D, 0xDD, 0xC1, 0x7C, 0xD0, 0xC4, 0x30, 0x41, 0x71, 0xC4, 0x1B, 0x51, 0x7F, 0x8, 0xBF, 0x5A, 0x7E, 0x10, 0xE6, 0x38, 0xE2, 0xE1, 0x52, 0x4B, 0x11, 0xD2]]);
    let recovered = rsa.decrypt_array_unit(&cipher);
    let mut rrr = [[0u128; 32]; 3];
    unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u8, rrr.as_mut_ptr() as *mut u8, 512 * 3); }
    
    print!("RSA_4096_u8: Recovered = [");
    for rr in rrr
    {
        print!("[");
        for r in rr
            { print!("{:#X}, ", r); }
        print!("], ");
    }
    println!("]");
    assert_eq!(rrr, [[0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6079889706A5B4C3D2E1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF, 0x_FEDCBA98765432100123456789ABCDEF, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_879605A4B3C2D1E00F1E2D3C4B5A6978, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_111122223333444499990000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_89706A5B4C3D2E1FF1E2D3C4B5A60798, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB, 0x_0123456789ABCDEFFEDCBA9876543210, 0x_556677889900AABB11223344CCDDEEFF, 0x_44332211FFEEDDCC88776655BBAA0099, 0x_4D3E2F11F29807A6B5CE3D4C5B6A7089, 0x_2D1E00F1E2D879605A4B3C3C4B5A6978, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_333344449999111122220000AAAABBBB, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000],
                    [0x_56789ABCDEF00FEDCBA9876543211234_u128, 0x_11223344CCDDEEFF556677889900AABB, 0x_FFEEDDCC5544332211BBAA0099887766, 0x_5B6A70899807A6B51F2E3D4CC4D3E2F1, 0x_B5A6079889706A5BF1E2D3C44C3D2E1F, 0x_102F3E4DD4E3F2015C6B7A8998A7B6C5, 0x_33334444555566661111222277778888, 0x_99990000EEEEFFFFAAAABBBBCCCCDDDD, 0x_FEDCBA99ABCDEF876543210012345678, 0x_CCDDEEFF112233449900AABB55667788, 0x_88776655BBAA009944332211FFEEDDCC, 0x_9807A6B5C5B6A7089C4D3E2F11F2E3D4, 0x_879605A43C4B5A6978B3C2D1E00F1E2D, 0x_E3F201102F3E98A7B6C5D44D5C6B7A89, 0x_111122220000AAAABBBB333344449999, 0x_5555666677778888CCCCDDDDEEEEFFFF, 0x_11111111111111111111111111111111, 0x_22222222222222222222222222222222, 0x_33333333333333333333333333333333, 0x_44444444444444444444444444444444, 0x_55555555555555555555555555555555, 0x_66666666666666666666666666666666, 0x_77777777777777777777777777777777, 0x_88888888888888888888888888888888, 0x_99999999999999999999999999999999, 0x_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, 0x_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, 0x_CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC, 0x_DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD, 0x_EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE, 0x_FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0x_00000000000000000000000000000000]]);
    assert_eq!(rrr, message);
    println!("-------------------------------");
}
