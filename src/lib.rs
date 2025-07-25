// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]

//! cryptocol crate provides libraries for cryptography.
//! 
//! This crate is optimized for Little-endian CPUs because Little-Endian CPUs
//! are far more popular than Big-endian CPUs. For the information about
//! Endianness (including Little-endian and Big-endian)
//! [Read more](https://en.wikipedia.org/wiki/Endianness).
//! 
//! # Big-endian issue
//! This crate is just experimental for Big-endian CPUs. So, you are not
//! encouraged to use this crate for Big-endian CPUs for serious purpose.
//! Only use this crate for Big-endian CPUs with your own full responsibility.
//! 
//! # Road Map for Version 1.0
//! This crate Cryptocol is planned to provide the following functionalities.
//! The checked items have already been implemented including documentation
//! at least 95%. The unchecked items have been implemented including
//! documentation less than 95% or have not yet even been started to implement.
//! 
//! ## Small Numbers: meaningful as itself, and also foundations mainly for Big Numbers as well as for other modules
//! - [X] Unions for primitive data types and their implementation, and the
//!       implementation of trait SmallUInt for the Unions ---
//!     [`ShortUnion`](number/short_union/union.ShortUnion.html#union.ShortUnion),
//!     [`IntUnion`](number/int_union/union.IntUnion.html#union.IntUnion),
//!     [`LongUnion`](number/long_union/union.LongUnion.html#union.LongUnion),
//!     [`LongerUnion`](number/longer_union/union.LongerUnion.html#union.LongerUnion),
//!     [`SizeUnion`](number/size_union/union.SizeUnion.html#union.SizeUnion),
//!     [`SharedValues`](number/shared_values/union.SharedValues.html#union.SharedValues), and
//!     [`SharedArrays`](number/shared_arrays/union.SharedArrays.html#union.SharedArrays)
//! - [X] Trait SmallUInt, its implementation for primitive data types, and the
//!       implementation of it for the Unions ---
//!     [`SmallUInt`](number/small_uint/trait.SmallUInt.html#trait.SmallUInt)
// ! - [ ] Trait SmallSInt, its implementation for primitive data types, and the
// !       implementation of it for the Unions ---
// !    [`SmallSInt`](number/small_sint/trait.SmallSInt.html#trait.SmallSInt)
// !     _--> Thinking about postponing to Roadmap for ver. 2.0_
//! 
//! ## Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms
//! - [X] Fixed Sized Big Unsigned Integer Operation --- You can find most of
//!       the methods you need for big integer calculation at
//!       [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt).
//! - [X] Auxiliary Fixed Sized Big Unsigned Integer Operations --- If you don't
//!       find what you need at
//!       [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt),
//!       you may find it at
//!       [`BigUInt_More`](number/trait_big_uint_more/trait.BigUInt_More.html#trait.BigUInt_More).
//!       You can't find modular-related methods at
//!       [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt),
//!       but you may find them at
//!       [`BigUInt_Modular`](number/trait_big_uint_modular/trait.BigUInt_Modular.html#trait.BigUInt_Modular).
//!       You can't find panic-free-releated methods at
//!       [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt),
//!       but you may find it at
//!       [`BigUInt_Panic_Free`](number/trait_big_uint_panic_free/trait.BigUInt_Panic_Free.html#trait.BigUInt_Panic_Free).
//!       You can't find prime number-related methods at
//!       [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt),
//!       you may find it at
//!       [`BigUInt_Prime`](number/trait_big_uint_prime/trait.BigUInt_Prime.html#trait.BigUInt_Prime).
// ! - [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
// !    _--> Thinking about postponing to Roadmap for ver. 2.0_
// ! - [ ] Variable Sized Big Signed Integer Operation --- LargeInt
// !    _--> Thinking about postponing to Roadmap for ver. 2.0 or higher_
//! 
//! ## Hash Algorithms
// ! - [ ] MD2 hash algorithms based on 128 bits
// !       --- Includes MD4 and its expanded versions.
// !       ===> Moved to Roadmap for ver. 2.0
//! - [X] MD4 hash algorithms based on 128 bits
//!       --- Includes MD4 and its expanded versions.
//!       [`MD4_Generic`](hash/md4/struct.MD4_Generic.html#struct.MD4_Generic)
//! - [X] MD5 hash algorithms based on 128 bits
//!       --- Includes MD5 and its expanded versions.
//!       [`MD5_Generic`](hash/md5/struct.MD5_Generic.html#struct.MD5_Generic)
// ! - [ ] MD6 hash algorithms based on 256 bits
// !       --- Includes MD4 and its expanded versions.
// !       ===> Moved to Roadmap for ver. 2.0
//! - [X] SHA-1 hash algorithms based on 160 bits
//!       --- Includes SHA-1, SHA-0, and their expanded versions.
//!       [`SHA1_Generic`](hash/sha1/struct.SHA1_Generic.html#struct.SHA1_Generic)
//! - [X] SHA-2 hash algorithms based on 256 bits
//!       --- Includes SHA-256, SHA-224, and their expanded versions.
//!       [`SHA2_256_Generic`](hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic)
//! - [X] SHA-2 hash algorithms based on 512 bits
//!       --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions.
//!       [`SHA2_512_Generic`](hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic)
//! - [X] SHA-2 hash algorithms based on 512/t bits
//!       --- Includes 512/256, SHA-512/224, and their expanded versions.
//!       [`SHA2_512_t_Generic`](hash/sha2_512_t/struct.SHA2_512_t_Generic.html#struct.SHA2_512_t_Generic)
//! - [X] SHA-3 and Keccak hash algorithms based on 8/16/32/64 bits
//!       --- Includes SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE 128, SHAKE 256, cSHAKE-128, cSHAKE-256, Keccak family and their expanded versions.
//!       [`Keccak_Generic`](hash/keccak/struct.Keccak_Generic.html#struct.Keccak_Generic)
// ! - [ ] RIPEMD hash algorithms based on 256 bits
// !       --- Includes RIPEMD and its expanded versions.
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] BLAKE2 hash algorithms based on 256 bits
// !       --- Includes BLAKE2 and its expanded versions.
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] BLAKE3 hash algorithms based on 256 bits
// !       --- Includes BLAKE3 and its expanded versions.
// !       ===> Moved to Roadmap for ver. 2.0
//! 
//! ## Symmetric-key Algorithms for the Encryption/Decryption of digital data
// ! - [ ] Lucifer symmetric-key encryption/decryption algorithm
// !       --- Includes Lucifer and its expanded versions. `Lucifer_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
//! - [X] DES symmetric-key encryption/decryption algorithm and the traits and its implementations of Operation modes and padding bits for DES_Generic
//!       --- Includes DES and its expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits
//!       according to PKCS#7 and ISO 7816-4.
//!       [`DES_Generic`](symmetric/des/struct.DES_Generic.html#struct.DES_Generic)
//!       [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
//!       [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
//!       [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
//!       [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
//!       [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
//!       [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
//!       [`CFB`](symmetric/trait.CFB.html#trait.CFB),
//!       [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
//!       [`CTR`](symmetric/trait.CTR.html#trait.CTR).
//! - [ ] AES and Rijdael symmetric-key encryption/decryption algorithm and the trait implementations of Operation modes and padding bits for AES_Generic
//!       --- Includes AES and its expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits
//!       according to PKCS#7 and ISO 7816-4.
//!       [`Rijdael_Generic`](symmetric/des/struct.Rijndael_Generic.html#struct.Rijndael_Generic),
//!       [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
//!       [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
//!       [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
//!       [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
//!       [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
//!       [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
//!       [`CFB`](symmetric/trait.CFB.html#trait.CFB),
//!       [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
//!       [`CTR`](symmetric/trait.CTR.html#trait.CTR).
//!       (symmetric/aes/struct.AES_Generic.html#struct.AES_Generic)
// ! - [ ] Bluefish symmetric-key encryption/decryption algorithm
// !     --- Includes Bluefish and its expanded versions. `Bluefish_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Twofish symmetric-key encryption/decryption algorithm
// !     --- Includes Twofish and its expanded versions. `Twofish_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] SEED symmetric-key encryption/decryption algorithm
// !     --- Includes SEED and its expanded versions. `SEED_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] HIGHT symmetric-key encryption/decryption algorithm
// !     --- Includes HIGHT and its expanded versions. `HIGHT_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] ARIA symmetric-key encryption/decryption algorithm
// !     --- Includes ARIA and its expanded versions. `ARIA_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] LEA symmetric-key encryption/decryption algorithm
// !     --- Includes LEA and its expanded versions. `LEA_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC2 symmetric-key encryption/decryption algorithm
// !     --- Includes RC2 and its expanded versions. `RC2_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC4 symmetric-key encryption/decryption algorithm
// !     --- Includes RC4 and its expanded versions. `RC4_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC5 symmetric-key encryption/decryption algorithm
// !     --- Includes RC5 and its expanded versions. `RC5_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC6 symmetric-key encryption/decryption algorithm
// !     --- Includes RC6 and its expanded versions. `RC6_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Salsa20 symmetric-key encryption/decryption algorithm
// !     --- Includes Salsa20 and its expanded versions. `Salsa20_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Chacha20 symmetric-key encryption/decryption algorithm
// !     --- Includes Chacha20 and its expanded versions. `Chacha20_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
// ! - [ ] IDEA symmetric-key encryption/decryption algorithm
// !     --- Includes IDEA and its expanded versions. `IDEA_Generic`
// !     ===> Moved to Roadmap for ver. 2.0
//!  - [ ] BigCryptor64 combinations of symmetric-key encryption/decryption algorithms and the trait implementations of Operation modes and padding bits for BigCryptor64
//!      --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4. `ECB_PKCS7`, `ECB_ISO`, `CBC_PKCS7`, `CBC_ISO`, `PCBC_PKCS7`, `PCBC_ISO`, `CFB`, `OFB`, and `CTR`.
//!  - [ ] BigCryptor128 combinations of symmetric-key encryption/decryption algorithms and the trait implementations of Operation modes and padding bits for BigCryptor128
//!      --- Includes 2AES, 3AES, 4AES, etc., and their expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4. `ECB_PKCS7`, `ECB_ISO`, `CBC_PKCS7`, `CBC_ISO`, `PCBC_PKCS7`, `PCBC_ISO`, `CFB`, `OFB`, and `CTR`.
//! 
//! 
//! ## Pseudo-Random Number Generator Algorithms
//! - [ ] Pseudo-random number generator --- struct
//!       [`Random_Generic`](random/random/struct.Random_Generic.html#struct.Random_Generic)
//!       and trait
//!       [Random_Engine](random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
//! - [ ] Pseudo-random number generator engines using hash algorithms ---
//!       [`Any_MD4`](random/random/struct.Any_MD4.html#struct.Any_MD4),
//!       [`Any_MD5`](random/random/struct.Any_MD5.html#struct.Any_MD5),
//!       [`Any_SHA0`](random/random/struct.Any_SHA0.html#struct.Any_SHA0),
//!       [`Any_SHA1`](random/random/struct.Any_SHA1.html#struct.Any_SHA1),
//!       [`Any_SHA2_256`](random/random/struct.Any_SHA2_256.html#struct.Any_SHA2_256),
//!       [`Any_SHA2_512`](random/random/struct.Any_SHA2_512.html#struct.Any_SHA2_512),
//!       [`Any_SHA3_256`](random/random/struct.Any_SHA3_256.html#struct.Any_SHA3_256),
//!       [`Any_SHA3_512`](random/random/struct.Any_SHA3_512.html#struct.Any_SHA3_512),
//!       [`Any_SHAKE_128`](random/random/struct.Any_SHAKE_128.html#struct.Any_SHAKE_128),
//!       [`Random_SHA2_512`](random/random/struct.Random_SHA2_512.html#struct.Random_SHA2_512).
//!       [`Random_SHA3_512`](random/random/struct.Random_SHA3_512.html#struct.Random_SHA3_512), and
//!       [`Random_BIG_KECCAK_1024`](random/random/struct.Random_BIG_KECCAK_1024.html#struct.Random_BIG_KECCAK_1024),
//! - [ ] Pseudo-random number generator engines using symmetric-key encryption algorithms ---
//!       [`Any_DES`](random/random/struct.Any_DES.html#struct.Any_DES),
//!       `Any_AES`, and
//!       `Random_AES`.
//! - [ ] Pseudo-random number generator engines using simple randomization algorithm
//!       --- [`Any_Num_C`](random/random/struct.Any_Num.html#struct.Any_Num_C)
//! 
//! ## Asymmetric-Key Algorithms for the Encryption/Decryption of digital data
// ! - [ ] Diffie-Hellman --> Thinking about postponing to Roadmap for ver. 2.0
// ! - [ ] ElGamal --> Moved to Roadmap for ver. 2.0
//! - [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
//! - [ ] ECC (Elliptic Curve Cryptosystem)
// ! - [ ] Rabin --> Moved to Roadmap for ver. 2.0
//! 
//! When the implementation of all the above functionalitis are completed,
//! the version number 1.0.0.0 will be given. After that whenever another
//! functionality is added to this crate, the version number will get higher
//! beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
//! number will be 0.21.x.x since there are all twenty-five functionalities
//! listed above. So, for example, even if the version number is 0.5.0.0,
//! it does not mean that 50% of all functionalities are implemented.


#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://rust-random.github.io/rand/"
)]

pub mod number;
pub mod hash;
pub mod symmetric;
pub mod random;
