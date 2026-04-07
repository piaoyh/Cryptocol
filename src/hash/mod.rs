// Copyright 2023, 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various cryptographic hash functions
//! 
//! # Introduction
//! The module that contains a few sub-modules
//! to define cryptographic hash functions
//! 
//! # Background: Cryptographic hash functions
//! What if we can make a kind of finger print of data and the finger print is
//! of small size? Then, we can identify data with that small-sized finger
//! print. And, by means of the finger print of the data, we can also determine
//! whether or not certain data has been changed. The finger print of data with
//! small size must be very useful. The output of the cryptographic hash
//! function is called hash value and can virtually play the role of the finger
//! print of data.
//! 
//! Cryptographic hash functions are hash algorithms that have three properties:
//! - Pre-image resistance;
//! - Second pre-image resistance; and
//! - Collision resistance.
//! 
//! Read [this article](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
//! and/or Watch [this lecture](https://www.youtube.com/watch?v=bDIc3jcLlOE)
//! or [this lecture](https://www.youtube.com/watch?v=tLkHk__-M6Q)
//! to learn cryptographic hash functions more in detail. However, the last
//! video lecture has a synch problem that the video and its audio are not
//! synchronized, though this lecture is so good that I would like to highly
//! encourage you to watch even with suffering from mis-synchronization.
//! 
//! # The algorithms of cryptographic hash functions
//! This module provides several kinds of cryptographic hash algorithms:
//! - SHA-3 and Keccak hash algorithms based on 8/16/32/64 bits
//!   --- Includes SHA3-224, SHA3-256, SHA3-384, SHA3-512,
//!       SHAKE 128, SHAKE 256, cSHAKE-128, cSHAKE-256,
//!       Keccak family and their expanded versions.
//!   [`Keccak_Generic`](struct@Keccak_Generic)
//! - SHA-2 hash algorithms based on 512/t bits
//!   --- Includes 512/256, SHA-512/224, and their expanded versions.
//!   [`SHA2_512_t_Generic`](struct@SHA2_512_t_Generic)
//! - SHA-2 hash algorithms based on 512 bits
//!   --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions.
//!   [`SHA2_512_Generic`](struct@SHA2_512_Generic)
//! - SHA-2 hash algorithms based on 256 bits
//!   --- Includes SHA-256, SHA-224, and their expanded versions.
//!   [`SHA2_Generic_256`](struct@SHA2_256_Generic)
//! - SHA-1 hash algorithms based on 160 bits
//!   --- Includes SHA-1, SHA-0, and their expanded versions.
//!   [`SHA1_Generic`](struct@SHA1_Generic)
// ! - RIPEMD hash algorithms based on 256 bits
// !   --- Includes RIPEMD and its expanded versions.
// !   ===> Moved to Roadmap for ver. 2.0
// ! - BLAKE3 hash algorithms based on 256 bits
// !   --- Includes BLAKE3 and its expanded versions.
// !   ===> Moved to Roadmap for ver. 2.0
// ! - BLAKE2 hash algorithms based on 256 bits
// !   --- Includes BLAKE2 and its expanded versions.
// !   ===> Moved to Roadmap for ver. 2.0
// ! - MD6 hash algorithms based on 256 bits
// !   --- Includes MD4 and its expanded versions.
// !   ===> Moved to Roadmap for ver. 2.0
//! - MD5 hash algorithms based on 128 bits
//!   --- Includes MD5 and its expanded versions.
//!   [`MD5_Generic`](struct@MD5_Generic)
//! - MD4 hash algorithms based on 128 bits
//!   --- Includes MD4 and its expanded versions.
//!   [`MD4_Generic`](struct@MD4_Generic)
// ! -  MD2 hash algorithms based on 128 bits
// !    --- Includes MD2 and its expanded versions.
// !    ===> Moved to Roadmap for ver. 2.0
//! 
//! 
//! # QUICK START
//! - For `Keccak`, read [here](struct@Keccak_Generic#quick-start).
//! - For `SHA-512/t`, read [here](struct@SHA2_512_t_Generic#quick-start).
//! - For `SHA-512`, read [here](struct@SHA2_512_Generic#quick-start).
//! - For `SHA-256`, read [here](struct@SHA2_256_Generic#quick-start).
//! - For `SHA-1`, read [here](struct@SHA1_Generic#quick-start).
//! - For `MD5`, read [here](struct@MD5_Generic#quick-start).
//! - For `MD4`, read [here](struct@MD4_Generic#quick-start).
//! 
//! # Simple but Useful Applications using cryptocol
//! - For `SHA3`, try [this](struct@Keccak_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-512/t`, try [this](struct@SHA2_512_t_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-512`, try [this](struct@SHA2_512_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-256`, try [this](struct@SHA2_256_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-1`, try [this](struct@SHA1_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `MD5`, try [this](struct@MD5_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `MD4`, try [this](struct@MD4_Generic#a-simple-but-useful-application-using-cryptocol).
//! 
//! The following application is made by combining above all applications.
//! 
//! ## A Simple but Useful Integrated Hash Application using cryptocol
//! The following is the source code of the commandline integrated hash value
//! extractor using the above modules. You can get the hash value from a text
//! or a file. The following source code assumes its executable file name will
//! be "hash_app". You can find all the examples including the following source
//! code in the folder "examples" of this crate.
//! ```
//! use std::{ io, env, fs };
//! use std::io::BufRead;
//! use std::convert::From;
//! use std::fmt::{ Debug, Display };
//! use cryptocol::number::SmallUInt;
//! use cryptocol::hash::*;
//! 
//! fn main()
//! {
//!     let args: Vec<String> = env::args().collect();
//!     if args.len() < 4  
//!     {
//!         help();
//!         return;
//!     }
//! 
//!     match &args[1][..]
//!     {
//!         "md4" => { print_hash_value(&args[2][..], MD4::new(), &args[3][..]); },
//!         "md5" => { print_hash_value(&args[2][..], MD5::new(), &args[3][..]); },
//!         "sha1" => { print_hash_value(&args[2][..], SHA1::new(), &args[3][..]); },
//!         "sha2_256" => { print_hash_value(&args[2][..], SHA2_256::new(), &args[3][..]); },
//!         "sha2_224" => { print_hash_value(&args[2][..], SHA2_224::new(), &args[3][..]); },
//!         "sha2_512" => { print_hash_value(&args[2][..], SHA2_512::new(), &args[3][..]); },
//!         "sha2_512_256" => { print_hash_value(&args[2][..], SHA2_512_256::new(), &args[3][..]); },
//!         "sha2_384" => { print_hash_value(&args[2][..], SHA2_384::new(), &args[3][..]); },
//!         "sha2_512_t_224" => { print_hash_value(&args[2][..], SHA2_512_t_224::new(), &args[3][..]); },
//!         "sha3" => { print_hash_value(&args[2][..], SHA3_256::new(), &args[3][..]); },
//!         _ => { help(); },
//!     }
//! }
//! 
//! fn print_hash_value<H: Hash>(text_or_file: &str, hash: H, src: &str)
//! {
//!     match text_or_file
//!     {
//!         "--text" | "-t" =>  { get_hash_value_from_text(hash, src); },
//!         "--file" | "-f" =>  { get_hash_value_from_file(hash, src); },
//!         "--check" | "-c" => { check_files(hash, src) },
//!         _ =>  { help(); },
//!     }
//! }
//! 
//! fn get_hash_value_from_text<H: Hash>(mut hash: H, txt: &str)
//! {
//!     hash.digest_str(txt);
//!     println!("Hash value:\t{}", hash.get_hash_value_in_string());
//! }
//! 
//! fn get_hash_value_from_file<H: Hash>(mut hash: H, file: &str)
//! {
//!     if let Ok(contents) = fs::read(file)
//!     {
//!         hash.digest_vec(&contents);
//!         println!("Hash value:\t{}", hash.get_hash_value_in_string());
//!     }
//!     else
//!     {
//!         println!("File Error!");
//!     }
//! }
//! 
//! fn check_files<H: Hash>(mut hash: H, file_list: &str)
//! {
//!     let mut reader;
//!     match fs::File::open(file_list)
//!     {
//!         Ok(file) => {
//!                 reader = io::BufReader::new(file);
//!                 let mut line = String::new();
//!                 while let Ok(n) = reader.read_line(&mut line)
//!                 {
//!                     if n == 0
//!                         { break; }
//!                     let txt = line.trim();
//!                     if txt.chars().nth(0).unwrap() == '#'
//!                     { 
//!                         line.clear();
//!                         continue;
//!                     }
//!                     let elem: Vec<&str> = txt.split_whitespace().collect();
//!                     let item = elem[0];
//!                     let h = String::from(elem[1]).to_uppercase();
//!                     if let Ok(contents) = fs::read(item)
//!                     {
//!                         hash.digest_vec(&contents);
//!                         if hash.get_hash_value_in_string() == h
//!                             { println!("{} ---> OK", item); }
//!                         else
//!                             { println!("{} ---> Corrupted", item); }
//!                     }
//!                     line.clear();
//!                 }
//!             },
//!         _ => {
//!                 println!("File open error");
//!                 return;
//!             }
//!     }
//! }
//! 
//! fn help()
//! {
//!     println!("This is a hash value extractor from a text or a file, using cryptocol.");
//!     println!("Usage: hash_app <algorithm> <option> <source>\n");
//!     println!("algorithms        description");
//!     println!("md4               : MD4 algorithm");
//!     println!("md5               : MD5 algorithm");
//!     println!("sha1              : SHA1 algorithm");
//!     println!("sha2_256          : SHA2_256 algorithm");
//!     println!("sha2_224          : SHA2_224 algorithm");
//!     println!("sha2_512          : SHA2_512 algorithm");
//!     println!("sha2_512_256      : SHA2_512_256 algorithm");
//!     println!("sha2_384          : SHA2_384 algorithm");
//!     println!("sha2_512_t_224    : SHA2_512_t_224 algorithm\n");
//!     println!("sha3              : SHA3_256 algorithm");
//!     println!("options           description");
//!     println!("--text, -t        : <source> is a text to get a hash code.");
//!     println!("                    The text should be enclosed by ' or \".");
//!     println!("--file, -f        : <source> is the name of the file to get a hash code.");
//!     println!("--check, -c       : <source> is the name of the file that contains pairs");
//!     println!("                    of file and its hash code.");
//!     println!("--help, -h        : print this help message on screen\n");
//!     println!("Examples:");
//!     println!("\thash_app md5 -t 'How are you doing?'");
//!     println!("\thash_app sha2_256 -f linuxmint-21.3-cinnamon-64bit.iso");
//!     println!("\thash_app sha3 -c CHECKSUM");
//! }
//! 
//! trait Hash
//! {
//!     fn digest_str(&mut self, message: &str);
//!     fn digest_vec<T>(&mut self, message: &Vec<T>) where T: SmallUInt + Copy + Clone + Display + Debug + ToString;
//!     fn get_hash_value_in_string(&self) -> String;
//! }
//! 
//! macro_rules! impl_hash_for
//! {
//!     ($($f:ty),*) => {
//!         $(
//!               impl Hash for $f
//!               {
//!                   #[inline] fn digest_str(&mut self, message: &str)   { self.digest_str(message); }
//!                   #[inline] fn digest_vec<T>(&mut self, message: &Vec<T>) where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!                               { self.digest_vec(message); }
//!                   #[inline] fn get_hash_value_in_string(&self) -> String  { self.get_hash_value_in_string() }
//!               }
//!         )* 
//!     };  
//! }
//! 
//! impl_hash_for!{ MD4, MD5, SHA1, SHA2_224, SHA2_256, SHA2_384,
//!                 SHA2_512, SHA2_512_256, SHA2_512_t_224, SHA3_256 }
//! ```

mod md4;
mod md5;
mod sha1;
mod sha2_256;
mod sha2_512;
mod sha2_512_t;
mod keccak;

pub use md4::{ MD4, MD4_Generic, MD4_Expanded, MD4_Generic_HR_fixed };
pub use md5::{ MD5, MD5_Generic, MD5_Expanded, MD5_Generic_HR_fixed };
pub use sha1::{ SHA1, SHA1_Generic, SHA1_Expanded, SHA1_Generic_HR_fixed,
                SHA0, SHA0_Expanded, SHA0_Generic_HR_fixed };
pub use sha2_256::{ SHA2_256, SHA2_256_Generic, SHA2_256_Expanded, SHA2_256_Generic_HRS_fixed,
                    SHA2_224, SHA2_224_Expanded, SHA2_224_Generic_HRS_fixed };
pub use sha2_512::{ SHA2_512, SHA2_512_Generic, SHA2_512_Expanded, SHA2_512_Generic_HRS_fixed,
                    SHA2_384, SHA2_384_Expanded, SHA2_384_Generic_HRS_fixed,
                    SHA2_512_256, SHA2_512_256_Expanded };
pub use sha2_512_t::{ SHA2_512_t, SHA2_512_t_Generic, SHA2_512_t_Expanded, SHA2_512_t_Generic_HRS_fixed,
                      SHA2_512_t_256, SHA2_512_t_256_Expanded, SHA2_512_t_224, SHA2_512_t_224_Expanded,
                      SHA2_512_0 };
pub use keccak::{ Keccak_Generic, BIG_KECCAK_1536, BIG_KECCAK_1024, BIG_KECCAK_768,
                  BIG_KECCAK_512, BIG_KECCAK_384, BIG_KECCAK_256, BIG_KECCAK_224, 
                  BIG_SHA3_1536, BIG_SHA3_1024, BIG_SHA3_768, BIG_SHA3_512, BIG_SHA3_384,
                  BIG_SHA3_256, BIG_SHA3_224,
                  BIG_SHAKE_1536, BIG_SHAKE_1024, BIG_SHAKE_768, BIG_SHAKE_512,
                  BIG_SHAKE_384, BIG_SHAKE_256, BIG_SHAKE_224, BIG_SHAKE_128,
                  BIG_cSHAKE_1536, BIG_cSHAKE_1024, BIG_cSHAKE_768, BIG_cSHAKE_512,
                  BIG_cSHAKE_384, BIG_cSHAKE_256, BIG_cSHAKE_224, BIG_cSHAKE_128,
                  KECCAK_768, KECCAK_512, KECCAK_384, KECCAK_256, KECCAK_224, 
                  SHA3_768, SHA3_512, SHA3_384, SHA3_256, SHA3_224,
                  SHAKE_768, SHAKE_512, SHAKE_384, SHAKE_256, SHAKE_224, SHAKE_128,
                  cSHAKE_768, cSHAKE_512, cSHAKE_384, cSHAKE_256, cSHAKE_224, cSHAKE_128,
                  SMALL_KECCAK_384, SMALL_KECCAK_256, SMALL_KECCAK_224, 
                  SMALL_SHA3_384, SMALL_SHA3_256, SMALL_SHA3_224,
                  SMALL_SHAKE_256, SMALL_SHAKE_224, SMALL_SHAKE_128,
                  SMALL_cSHAKE_256, SMALL_cSHAKE_224, SMALL_cSHAKE_128,
                  SMALLER_KECCAK_128, SMALLER_SHA3_128, SMALLER_SHAKE_128, SMALLER_cSHAKE_128,
                  TINY_KECCAK_64, TINY_SHA3_64, TINY_SHAKE_64, TINY_cSHAKE_64 };

/// many *.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
mod documentation;
pub use documentation::*;