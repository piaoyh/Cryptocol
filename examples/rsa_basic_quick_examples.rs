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

use cryptocol::number::BigUInt_Modular;



pub fn main()
{
    rsa_quick_start();
    rsa_new();
    // rsa_decrypt();
    // rsa_same_primes_test();
    // rsa_composites_test();
    // rsa_composite_prime_test();
}

fn rsa_quick_start()
{
    rsa_import();
}

fn rsa_import()
{
    println!("rsa_import");
    use cryptocol::asymmetric::RSA_4096;
    use cryptocol::asymmetric::RSA_2048;
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::asymmetric::RSA_Generic;
    println!("-------------------------------");
}

fn rsa_new()
{
    println!("rsa_encrypt");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    // Example for RSA_4096
    use cryptocol::asymmetric::RSA_4096;
    let rsa = RSA_4096::new();
    
    // Example for RSA_2048
    use cryptocol::asymmetric::RSA_2048;
    let rsa = RSA_2048::new();
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    let rsa = RSA_1024::new();
    
    // Example for RSA_Genric
    use cryptocol::asymmetric::RSA_Genric;
    let rsa = RSA_Genric::new();
    println!("-------------------------------");
}