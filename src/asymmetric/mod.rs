// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// ! various asymmetric-key algorithms for the encryption/decryption of digital data
// ! 
// ! # Introduction
// ! The module that contains a few sub-modules to define symmetric-key
// ! algorithms for the encryption/decryption of digital data
// ! 
// ! # Background: Symmetric encryption/decryption algorithms
// ! There are two caregories in encryption/decryption algorithms:
// ! Symmetric-Key cryptographic algorithms and Asymmetric-Key cryptographic
// ! algorithms. The Symmetric-Key cryptographic algorithms use the same keys
// ! to encrypt and to decrypt, and are way faster than the Symmetric-Key
// ! cryptographic algorithms. This module implements a few Symmetric-Key
// ! cryptographic algorithms in Rust codes.
// ! 
// ! Read [this article](https://en.wikipedia.org/wiki/Symmetric-key_algorithm)
// ! and/or Watch [this lecture](https://www.youtube.com/watch?v=sx1jUqdNxQc)
// ! to learn symmetric encryption/decryption algorithms more in detail.
// ! 
// ! # The symmetric-key algorithms for the encryption/decryption of digital data
// !   This module provides several kinds of symmetric-key algorithms for the
// !   encryption/decryption of digital data:
// ! - AES and Rijndael symmetric-key encryption/decryption algorithm and the
// !   trait implementations of Operation modes and padding bits for AES_Generic
// !   --- Includes AES and its expanded versions, and ECB, CBC, PCBC, CFB, OFB,
// !   and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
// !   [`Rijndael_Generic`](struct@Rijndael_Generic),
// !   [`ECB_PKCS7`](trait@ECB_PKCS7),
// !   [`ECB_ISO`](trait@ECB_ISO),
// !   [`CBC_PKCS7`](trait@ECB_PKCS7),
// !   [`CBC_ISO`](trait@CBC_ISO),
// !   [`PCBC_PKCS7`](trait@PCBC_PKCS7),
// !   [`PCBC_ISO`](trait@PCBC_ISO),
// !   [`CFB`](trait@CFB),
// !   [`OFB`](trait@OFB), and
// !   [`CTR`](trait@CTR).
// ! - [ ] MARS symmetric-key encryption/decryption algorithm
// !       --- Includes MARS and its expanded versions. `MARS_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Serpent symmetric-key encryption/decryption algorithm
// !       --- Includes RC6 and its expanded versions. `RC6_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Twofish symmetric-key encryption/decryption algorithm
// !       --- Includes Twofish and its expanded versions. `Twofish_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC6 symmetric-key encryption/decryption algorithm
// !       --- Includes RC6 and its expanded versions. `RC6_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] SEED symmetric-key encryption/decryption algorithm
// !       --- Includes SEED and its expanded versions. `SEED_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] HIGHT symmetric-key encryption/decryption algorithm
// !       --- Includes HIGHT and its expanded versions. `HIGHT_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] ARIA symmetric-key encryption/decryption algorithm
// !       --- Includes ARIA and its expanded versions. `ARIA_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] LEA symmetric-key encryption/decryption algorithm
// !       --- Includes LEA and its expanded versions. `LEA_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] IDEA symmetric-key encryption/decryption algorithm
// !       --- Includes IDEA and its expanded versions. `IDEA_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Bluefish symmetric-key encryption/decryption algorithm
// !       --- Includes Bluefish and its expanded versions. `Bluefish_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Chacha20 symmetric-key encryption/decryption algorithm
// !       --- Includes Chacha20 and its expanded versions. `Chacha20_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] Salsa20 symmetric-key encryption/decryption algorithm
// !       --- Includes Salsa20 and its expanded versions. `Salsa20_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC5 symmetric-key encryption/decryption algorithm
// !       --- Includes RC5 and its expanded versions. `RC5_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC4 symmetric-key encryption/decryption algorithm
// !       --- Includes RC4 and its expanded versions. `RC4_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] RC2 symmetric-key encryption/decryption algorithm
// !       --- Includes RC2 and its expanded versions. `RC2_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - DES symmetric-key encryption/decryption algorithm and the traits and
// !   its implementations of Operation modes and padding bits for DES_Generic
// !   --- Includes DES and its expanded versions, and ECB, CBC, PCBC, CFB, OFB,
// !   and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
// !   [`DES_Generic`](struct@DES_Generic),
// !   [`ECB_PKCS7`](trait@ECB_PKCS7),
// !   [`ECB_ISO`](trait@ECB_ISO),
// !   [`CBC_PKCS7`](trait@ECB_PKCS7),
// !   [`CBC_ISO`](trait@CBC_ISO),
// !   [`PCBC_PKCS7`](trait@PCBC_PKCS7),
// !   [`PCBC_ISO`](trait@PCBC_ISO),
// !   [`CFB`](trait@CFB),
// !   [`OFB`](trait@OFB), and
// !   [`CTR`](trait@CTR).
// ! - [ ] Lucifer symmetric-key encryption/decryption algorithm
// !       --- Includes Lucifer and its expanded versions. `Lucifer_Generic`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - BigCryptor128 combinations of symmetric-key encryption/decryption
// !   algorithms and the trait implementations of Operation modes and padding
// !   bits for BigCryptor128
// !   --- Includes 2AES, 3AES, 4AES, etc., and their expanded versions, and ECB,
// !   CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7
// !   and ISO 7816-4.
// !   [`BigCryptor128`](struct@BigCryptor128)
// !   [`ECB_PKCS7`](trait@ECB_PKCS7),
// !   [`ECB_ISO`](trait@ECB_ISO),
// !   [`CBC_PKCS7`](trait@ECB_PKCS7),
// !   [`CBC_ISO`](trait@CBC_ISO),
// !   [`PCBC_PKCS7`](trait@PCBC_PKCS7),
// !   [`PCBC_ISO`](trait@PCBC_ISO),
// !   [`CFB`](trait@CFB),
// !   [`OFB`](trait@OFB), and
// !   [`CTR`](trait@CTR).
// !   However, it is considered that 2AES, 3AES, 4AES, etc. are not very
// !   meaningful because AES-256, Rijndael_128_384, Rijndael_128_512, etc. are
// !   considered to be better than 2AES, 3AES, 4AES, etc.
// ! - BigCryptor64 combinations of symmetric-key encryption/decryption
// !   algorithms and the trait implementations of Operation modes and padding
// !   bits for BigCryptor64
// !   --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions, and
// !   ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to
// !   PKCS#7 and ISO 7816-4.
// !   [`BigCryptor64`](struct@BigCryptor64)
// !   [`BigCryptor128`](struct@BigCryptor128)
// !   [`ECB_PKCS7`](trait@ECB_PKCS7),
// !   [`ECB_ISO`](trait@ECB_ISO),
// !   [`CBC_PKCS7`](trait@ECB_PKCS7),
// !   [`CBC_ISO`](trait@CBC_ISO),
// !   [`PCBC_PKCS7`](trait@PCBC_PKCS7),
// !   [`PCBC_ISO`](trait@PCBC_ISO),
// !   [`CFB`](trait@CFB),
// !   [`OFB`](trait@OFB), and
// !   [`CTR`](trait@CTR).
// ! 
// ! 
// ! # QUICK START
// ! - For `AES` or `Rijndael`, read [here](struct@Rijndael_Generic#quick-start).
// ! - For `DES`, read [here](struct@DES_Generic#quick-start).
// ! - For `BigCryptor128`, read [here](struct@BigCryptor128#quick-start).
// ! - For `BigCryptor64`, read [here](struct@BigCryptor64#quick-start).



mod rsa;

// mod operation_mode_macros;
// mod trait_ecb_with_padding_pkcs7;
// mod trait_ecb_with_padding_iso;
// mod trait_cbc_with_padding_pkcs7;
// mod trait_cbc_with_padding_iso;
// mod trait_pcbc_with_padding_pkcs7;
// mod trait_pcbc_with_padding_iso;
// mod trait_cfb;
// mod trait_ofb;
// mod trait_ctr;
// mod trait_small_cryptor;

// mod trait_ecb_with_padding_pkcs7_impl_for_des;
// mod trait_ecb_with_padding_pkcs7_impl_for_rijndael;
// mod trait_ecb_with_padding_pkcs7_impl_for_big_cryptor64;
// mod trait_ecb_with_padding_pkcs7_impl_for_big_cryptor128;
// mod trait_ecb_with_padding_iso_impl_for_des;
// mod trait_ecb_with_padding_iso_impl_for_rijndael;
// mod trait_ecb_with_padding_iso_impl_for_big_cryptor64;
// mod trait_ecb_with_padding_iso_impl_for_big_cryptor128;
// mod trait_cbc_with_padding_pkcs7_impl_for_des;
// mod trait_cbc_with_padding_pkcs7_impl_for_rijndael;
// mod trait_cbc_with_padding_pkcs7_impl_for_big_cryptor64;
// mod trait_cbc_with_padding_pkcs7_impl_for_big_cryptor128;
// mod trait_cbc_with_padding_iso_impl_for_des;
// mod trait_cbc_with_padding_iso_impl_for_rijndael;
// mod trait_cbc_with_padding_iso_impl_for_big_cryptor64;
// mod trait_cbc_with_padding_iso_impl_for_big_cryptor128;
// mod trait_pcbc_with_padding_pkcs7_impl_for_des;
// mod trait_pcbc_with_padding_pkcs7_impl_for_rijndael;
// mod trait_pcbc_with_padding_pkcs7_impl_for_big_cryptor64;
// mod trait_pcbc_with_padding_pkcs7_impl_for_big_cryptor128;
// mod trait_pcbc_with_padding_iso_impl_for_des;
// mod trait_pcbc_with_padding_iso_impl_for_rijndael;
// mod trait_pcbc_with_padding_iso_impl_for_big_cryptor64;
// mod trait_pcbc_with_padding_iso_impl_for_big_cryptor128;
// mod trait_cfb_impl_for_des;
// mod trait_cfb_impl_for_rijndael;
// mod trait_cfb_impl_for_big_cryptor64;
// mod trait_cfb_impl_for_big_cryptor128;
// mod trait_ofb_impl_for_des;
// mod trait_ofb_impl_for_rijndael;
// mod trait_ofb_impl_for_big_cryptor64;
// mod trait_ofb_impl_for_big_cryptor128;
// mod trait_ctr_impl_for_des;
// mod trait_ctr_impl_for_rijndael;
// mod trait_ctr_impl_for_big_cryptor64;
// mod trait_ctr_impl_for_big_cryptor128;
// mod trait_small_cryptor64_impl_for_des;
// mod trait_small_cryptor64_impl_for_rijndael;
// mod trait_small_cryptor64_impl_for_big_cryptor64;
// mod trait_small_cryptor128_impl_for_rijndael;
// mod trait_small_cryptor128_impl_for_big_cryptor128;
// mod trait_for_big_cryptor_impl;

pub use rsa::*;
// pub use rijndael::*;
// pub use big_cryptor64::*;
// pub use big_cryptor128::*;
// use operation_mode_macros::*;

// pub use trait_ecb_with_padding_pkcs7::ECB_PKCS7;
// pub use trait_ecb_with_padding_iso::ECB_ISO;
// pub use trait_cbc_with_padding_pkcs7::CBC_PKCS7;
// pub use trait_cbc_with_padding_iso::CBC_ISO;
// pub use trait_pcbc_with_padding_pkcs7::PCBC_PKCS7;
// pub use trait_pcbc_with_padding_iso::PCBC_ISO;
// pub use trait_cfb::CFB;
// pub use trait_ofb::OFB;
// pub use trait_ctr::CTR;
// pub use trait_small_cryptor::SmallCryptor;


// /// many *.rs was too big because of documentation and plenty of examples
// /// So, in order to provide documentation without `docs.rs`'s failing
// /// generating documentation, dummy codes were made and documentation and
// /// examples were moved to all the *.rs in documentation folder.
// mod documentation;
// pub use documentation::*;
