// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::vec::Vec;
use crate::number::{ LongUnion, LongerUnion, SmallUInt };
use crate::symmetric::{ DES, SmallCryptor };



/// A BigCryptor64 wrapper struct for cascading encryption/decryption
/// algorithms that has 64-bit block size
/// 
/// # Introduction
/// BigCryptor64 is mainly used to make TDES which folds DES by cascading DES
/// three times. However, it can be used to cascade any encryption/decryption
/// algorithms that has 64-bit block size.
///
/// # Quick Start
/// You have to import (use) the module `BigCryptor64` in order to cascade
/// encryption/decryption algorithms that has 64-bit block size such as DES
/// as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::BigCryptor64;
/// ```
/// 
/// You can instantiate the BigCryptor64 object with components (hereinafter,
/// referred to as small cryptor) that has `u64` keys as Example 2,
/// for example. In this case, you have to take endianness into account.
/// In little-endianness, 0x_1234567890ABCDEF_u64 is [0xEFu8, 0xCDu8, 0xABu8,
/// 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8] while the same 
/// 0x_1234567890ABCDEF_u64 is [0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x90u8, 0xABu8,
/// 0xCDu8, 0xEF_u64] in big-endianness.
/// The instantiated object should be mutable.
/// 
/// # Example 2
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut _tdes = BigCryptor64::new()
///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
///
/// ```
/// 
/// Also, you can instantiate the BigCryptor64 object with small cryptors
/// (component) that have `[u8; 8]` keys as shown in Example 3. In this case,
/// you don't have to take endianness into account. The instantiated object
/// should be mutable.
/// 
/// # Example 3
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut _tdes = BigCryptor64::new()
///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// ```
/// 
/// You can instantiate the BigCryptor64 object without small cryptors
/// (components) that have key and set a `u64` key later as shown
/// in Example 4 or `[u8; 8]` keys later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut tdes = BigCryptor64::new();
/// let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
/// let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
/// let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
/// tdes.push_small_cryptor(des1);
/// tdes.push_small_cryptor(des2);
/// tdes.push_small_cryptor(des3);
/// ```
/// 
/// # Example 5
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut tdes = BigCryptor64::new();
/// let des1 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// let des2 = DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// let des3 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// tdes.push_small_cryptor(des1);
/// tdes.push_small_cryptor(des2);
/// tdes.push_small_cryptor(des3);
/// ```
/// 
/// Now, you can freely use any operation mode. This crate provide
/// ECB (Electronic CodeBook), CBC(Cipher Block Chaining), PCBC (Propagation
/// Cipher Block Chaining), CFB (Cipher FeedBack) OFB (Output FeedBack), and
/// CTR (CounTeR). You can choose the way of padding bits according to either
/// [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/) or
/// [ISO 7816-4](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
/// So, you can import (use) one of the following traits: ECB_PKCS7, ECB_ISO,
/// CBC_PKCS7, CBC_ISO, PCBC_PKCS7, PCBC_ISO, CFB, OFB, and CTR. The following
/// example 6 shows the case that you choose CBC operation mode and padding bits
/// according to PKCS #7.
/// 
/// # Example 6
/// ```
/// use std::fmt::Write as _;
/// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
/// let mut tdes = BigCryptor64::new()
///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// 
/// let iv = 0x_FEDCBA0987654321_u64;
/// println!("IV =	{:#018X}", iv);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let mut cipher = Vec::<u8>::new();
/// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "86 2B D7 BF 00 2E CD 70 ED 0C E3 8D 75 18 CE 0F BD A7 AE AF E5 19 46 F8 15 7A 24 0E CB 20 91 C0 03 B9 56 C5 77 01 33 E8 8E 84 CA B9 F2 99 63 AC 3A 3D 1F EF CA CA CB 67 ");
///
/// let mut recovered = String::new();
/// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
/// println!("B =\t{}", recovered);
/// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
/// assert_eq!(recovered, message);
/// ```
/// 
/// # Notice for Practical Use
/// Now, you can freely use any methods with any paddings
/// in any operation modes.
/// - This crate provides six operation modes:
///   ECB, CBC, PCBC, CFB, OFB, and CTR.
/// - This crate provides two padding ways: ISO 7816-4 and PKCS #7.
/// - The operation modes ECB, CBC and PCBC requires padding bytes.
/// - You can combine three operation modes and two padding ways.
/// - The operation modes
///   [`CFB`](./trait.CFB.html#trait.CFB),
///   [`OFB`](./trait.OFB.html#trait.OFB), and
///   [`CTR`](./trait.CTR.html#trait.CTR)
///   does not require padding bytes.
/// - The traits that implements combination of operation modes and padding
///   ways are provided such as
///   [`ECB_PKCS7`](./trait.ECB_PKCS7.html#trait.ECB_PKCS7),
///   [`ECB_ISO`](./trait.ECB_ISO.html#trait.ECB_ISO),
///   [`CBC_PKCS7`](./trait.CBC_PKCS7.html#trait.ECB_PKCS7),
///   [`CBC_ISO`](./trait.CBC_ISO.html#trait.CBC_ISO),
///   [`PCBC_PKCS7`](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7), and
///   [`PCBC_ISO`](./trait.PCBC_ISO.html#trait.PCBC_ISO).
/// - You can find detaild instructions and their helpful examples
///   if you go through those links.
/// 
/// In summary,
/// 
/// |      | padding PKCS7                                          | padding ISO                                      | no padding                        |
/// |------|--------------------------------------------------------|--------------------------------------------------|-----------------------------------|
/// | ECB  | [ECB_PKCS7](./trait.ECB_PKCS7.html#trait.ECB_PKCS7)    | [ECB_ISO](./trait.ECB_ISO.html#trait.ECB_ISO)    |                                   |
/// | CBC  | [CBC_PKCS7](./trait.CBC_PKCS7.html#trait.ECB_PKCS7)    | [CBC_ISO](./trait.CBC_ISO.html#trait.CBC_ISO)    |                                   |
/// | PCBC | [PCBC_PKCS7](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7) | [PCBC_ISO](./trait.PCBC_ISO.html#trait.PCBC_ISO) |                                   |
/// | CFB  |                                                        |                                                  | [CFB](./trait.CFB.html#trait.CFB) |
/// | OFB  |                                                        |                                                  | [OFB](./trait.OFB.html#trait.OFB) |
/// | CTR  |                                                        |                                                  | [CTR](./trait.CTR.html#trait.CTR) |
/// 
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    block: LongUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>,
    enc: fn (s: &mut Self, message: u64) -> u64,
    dec: fn (s: &mut Self, cipher: u64) -> u64,
}

impl BigCryptor64
{
    pub(super) const BLOCK_SIZE: usize = 8;
    const SUCCESS: u64 = !0;
    const FAILURE: u64 = 0;

    // pub fn new() -> Self
    /// Constructs a new object BigCryptor64.
    /// 
    /// # Output
    /// A new object BigCryptor64 that has no small cryptors by default
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method does not set any small cryptor (component) by default.
    /// - You have to set as many small cryptors (components) as you want.
    /// - The small cryptors (components) should have the block size 64-bit.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// let mut tdes = BigCryptor64::new();
    /// tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// tdes.push_small_cryptor(DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new)           
    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            block:          LongUnion::zero(),
            smallcryptor:   Vec::new(),
            enc:            Self::encrypt_u64,
            dec:            Self::decrypt_u64,
        }
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    /// Constructs a new object BigCryptor64 with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// `smallcryptor` is an array of small cryptors (components)
    ///  wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor64 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// let mut _tdes = BigCryptor64::new_with_small_cryptor_array(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_small_cryptor_array)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    /// Constructs a new object `BigCryptor64` with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// `smallcryptor` is a Vec object of small cryptors (components)
    ///  wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// let mut _tdes = BigCryptor64::new_with_small_cryptor_vec(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_small_cryptor_vec)
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    // pub fn push_small_cryptor<S: SmallCryptor<u64, 8> + 'static>(&mut self, smallcryptor: S)
    /// Adds a small cryptor (component) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptor` is a small cryptors (components).
    /// 
    /// # Features
    /// This method automatically wrap the small cryptor by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
    /// let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// tdes.push_small_cryptor(des1);
    /// tdes.push_small_cryptor(des2);
    /// tdes.push_small_cryptor(des3);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor)
    pub fn push_small_cryptor<S: SmallCryptor<u64, 8> + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
    }

    // pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptors: [Box<dyn SmallCryptor<u64, 8>>; N])
    /// Adds small cryptors (components) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptors` is an array of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the array the small cryptors should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// tdes.push_small_cryptor_array(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor_array)
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptors: [Box<dyn SmallCryptor<u64, 8>>; N])
    {
        for val in smallcryptors
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec<const N: usize>(&mut self, smallcryptor: [SmallCryptor; N])
    /// Adds small cryptors (components) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptors` is a Vec object of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the Vec object of the small cryptors
    /// should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// tdes.push_small_cryptor_vec(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor_vec)
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>)
    {
        self.smallcryptor = smallcryptor;
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor64.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor64,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for embracing BigCryptor64,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// tdes.turn_inverse();
    /// let mut bigcryptor = des + rijndael + tdes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u64(plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_0036D446DF6D218F_u64);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    /// println!("Recovered text:\t{:#018X}", recovered_text);
    /// assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.turn_inverse)
    #[inline]
    pub fn turn_inverse(&mut self)
    {
        (self.enc, self.dec) = (self.dec, self.enc);
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor64 to encryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor64,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for embracing BigCryptor64,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// tdes.turn_encryptor();
    /// let mut bigcryptor = des + rijndael + tdes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u64(plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_911ED9892E52BC7C_u64);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    /// println!("Recovered text:\t{:#018X}", recovered_text);
    /// assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.turn_encryptor)
    pub fn turn_encryptor(&mut self)
    {
        self.enc = Self::encrypt_u64;
        self.dec = Self::decrypt_u64;
    }

    // pub fn turn_decryptor(&mut self)
    /// Changes its role in BigCryptor64 to decryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor64,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for embracing BigCryptor64,
    ///   it will not be changed at all.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// tdes.turn_decryptor();
    /// let mut bigcryptor = des + rijndael + tdes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u64(plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_0036D446DF6D218F_u64);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    /// println!("Recovered text:\t{:#018X}", recovered_text);
    /// assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.turn_decryptor)
    pub fn turn_decryptor(&mut self)
    {
        self.enc = Self::decrypt_u64;
        self.dec = Self::encrypt_u64;
    }

    // pub fn encrypt_u64(&mut self, message: u64) -> u64
    /// Encrypts a 64-bit data.
    /// 
    /// # Arguments
    /// `message` is of `u64`-type and the plaintext to be encrypted.
    /// 
    /// # Output
    /// This method returns the encrypted data of `u64`-type from `message`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = 0x1234567890ABCDEF_u64;
    /// println!("M = {:#018X}", message);
    /// let cipher = tdes.encrypt_u64(message);
    /// println!("C = {:#018X}", cipher);
    /// assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_u64)
    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        self.set_block(message);
        self.encrypt_block();
        self.block.get()
    }

    // pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    /// Decrypts a 64-bit data.
    /// 
    /// # Arguments
    /// `cioher` is of `u64`-type and the ciphertext to be decrypted.
    /// 
    /// # Output
    /// This method returns the decrypted data of `u64`-type from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = 0x1234567890ABCDEF_u64;
    /// println!("M = {:#018X}", message);
    /// let cipher = tdes.encrypt_u64(message);
    /// println!("C = {:#018X}", cipher);
    /// assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);
    /// 
    /// let recovered = tdes.decrypt_u64(cipher);
    /// println!("B = {:#018X}", recovered);
    /// assert_eq!(recovered, 0x1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_u64)
    pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.block.get()
    }

    #[inline]
    pub(super) fn _encrypt(&mut self, message: u64) -> u64
    {
        (self.enc)(self, message)
    }

    #[inline]
    pub(super) fn _decrypt(&mut self, cipher: u64) -> u64
    {
        (self.dec)(self, cipher)
    }

    // pub fn encrypt_array_u64<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    /// Encrypts an array of 64-bit data.
    /// 
    /// # Arguments
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   stored.
    /// 
    /// # Output
    /// This method returns the encrypted data of an array of `u64`-type
    /// from `message`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#018X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u64; 3];
    /// tdes.encrypt_array_u64(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    /// assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    /// assert_eq!(cipher[2], 0x_51712805A458A102_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_array_u64)
    pub fn encrypt_array_u64<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    {
        for i in 0..M
            { cipher[i] = self.encrypt_u64(message[i]); }
    }

    // pub fn decrypt_array_u64<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
    /// Decrypts an array of 64-bit data.
    /// 
    /// # Arguments
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   stored.
    /// 
    /// # Output
    /// This method returns the decrypted data of an array of `u64`-type
    /// from `cipher`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#018X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u64; 3];
    /// tdes.encrypt_array_u64(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    /// assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    /// assert_eq!(cipher[2], 0x_51712805A458A102_u64);
    /// 
    /// let mut recovered = [0_u64; 3];
    /// tdes.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B = ");
    /// for r in recovered.clone()
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_1122334455667788_u64);
    /// assert_eq!(recovered[2], 0x_9900AABBCCDDEEFF_u64);
    /// assert_eq!(recovered[0], message[0]);
    /// assert_eq!(recovered[1], message[1]);
    /// assert_eq!(recovered[2], message[2]);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_array_u64)
    pub fn decrypt_array_u64<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
    {
        for i in 0..M
            { message[i] = self.decrypt_u64(cipher[i]); }
    }

    // pub fn is_succeful(&self) -> bool
    /// Checks whether the previous encryption or decryption was successful.
    /// 
    /// # Output
    /// If the previous encryption or decryption was successful, this method
    /// returns true. Otherwise, it returns false.
    /// 
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 8 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    /// 
    /// # Example 1 for Successful case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let success = tdes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_successful)
    #[inline] pub fn is_successful(&self) -> bool
    {
        self.block.get() == Self::SUCCESS
    }

    // pub fn is_failed(&self) -> bool
    /// Checks whether the previous encryption or decryption was failed.
    /// 
    /// # Output
    /// If the previous encryption or decryption was failed, this method
    /// returns true. Otherwise, it returns false.
    /// 
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 8 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    /// 
    /// # Example 1 for Successful case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let failure = tdes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_failed)
    #[inline] pub fn is_failed(&self) -> bool
    {
        self.block.get() == Self::FAILURE
    }

    #[inline] pub(crate) fn get_smallcryptor(&self) -> &Vec<Box<dyn SmallCryptor<u64, 8>>>  { &self.smallcryptor }

    pub(crate) fn move_to_next_key(&mut self)
    {
        let len = self.smallcryptor.len();
        for i in 0..len
            { self.smallcryptor[i].move_to_next_key(); }
    }

    // pub(super) fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```comfile_fail
    /// use cryptocol::symmetric::DES;
    /// let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// assert_eq!(a_des.is_successful(), false);
    /// 
    /// a_des.set_successful();
    /// assert_eq!(a_des.is_successful(), true);
    /// ```
    #[inline]
    pub(super) fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    // pub(super) fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```compile_fail
    /// use cryptocol::symmetric::DES;
    /// let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// a_des.encrypt_u64(0x1234567890ABCDEF_u64);
    /// assert_eq!(a_des.is_failed(), false);
    /// 
    /// a_des.set_failed();
    /// assert_eq!(a_des.is_failed(), true);
    /// ```
    #[inline]
    pub(super) fn set_failed(&mut self)
    {
        self.block.set(Self::FAILURE);
    }

    fn encrypt_block(&mut self)
    {
        let mut block = self.smallcryptor[0].encrypt_unit(self.block.get());
        for i in 1..self.smallcryptor.len()
            { block = self.smallcryptor[i].encrypt_unit(block); }
        self.block.set(block);
    }

    fn decrypt_block(&mut self)
    {
        let len = self.smallcryptor.len();
        let mut block = self.smallcryptor[len-1].decrypt_unit(self.block.get());
        for i in 2..len+1
            { block = self.smallcryptor[len-i].decrypt_unit(block); }
        self.block.set(block);
    }

    #[inline] fn get_block(&self) -> u64            { self.block.get() }
    #[inline] fn set_block(&mut self, block: u64)   { self.block.set(block); }
}


/// struct that generates BigCryptor64 with three DES objects
pub struct TDES {}

impl TDES
{
    // pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with three DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first and third DES objects, and is of `u64`.
    /// - `key2` is a key for the second DES object, and is of `u64`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with three DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first and third DES objects are encryptors
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ TDES, CBC_PKCS7 };
    /// 
    /// let mut tdes = TDES::new_with_2_keys_u64(0x_1234567890ABCDEF_u64, 0x_FEDCBA0987654321_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
                            + DES::encryptor_with_key_u64(key1)
    }

    // pub fn new_with_3_keys_u64(key1: u64, key2: u64, key3: u64) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with three DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first DES object, and is of `u64`.
    /// - `key2` is a key for the second DES object, and is of `u64`.
    /// - `key1` is a key for the third DES object, and is of `u64`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with three DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first and third DES objects are encryptors
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ TDES, CBC_PKCS7 };
    /// 
    /// let mut tdes = TDES::new_with_3_keys_u64(0x_1234567890ABCDEF_u64, 0x_1122334455667788_u64, 0x_9900AABBCCDDEEFF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B4 6C 2D 9A F5 67 60 45 A0 2B 68 E8 76 8E BD EF 7E 2D 7A 49 9A DE 43 0D 4C 3C 50 1E B6 8B 0A E8 A4 88 6E E7 FF 99 B9 2D 83 67 C9 3C 4A 2D C7 BA 40 F3 19 88 05 F2 2C D9 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_3_keys_u64(key1: u64, key2: u64, key3: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
                            + DES::encryptor_with_key_u64(key3)
    }

    // pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with three DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first and third DES objects,
    ///   and is of `[u8; 8]`.
    /// - `key2` is a key for the second DES object, and is of `[u8; 8]`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with three DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first and third DES objects are encryptors
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ TDES, CBC_PKCS7 };
    /// 
    /// let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// let key2 = [0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21];
    /// let mut tdes = TDES::new_with_2_keys(key1, key2);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 2B D7 BF 00 2E CD 70 ED 0C E3 8D 75 18 CE 0F BD A7 AE AF E5 19 46 F8 15 7A 24 0E CB 20 91 C0 03 B9 56 C5 77 01 33 E8 8E 84 CA B9 F2 99 63 AC 3A 3D 1F EF CA CA CB 67 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1.clone())
                            + DES::decryptor_with_key(key2)
                            + DES::encryptor_with_key(key1)
    }

    // pub fn new_with_3_keys(key1: [u8; 8], key2: [u8; 8], key3: [u8; 8]) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with three DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first and third DES objects,
    ///   and is of `[u8; 8]`.
    /// - `key2` is a key for the second DES object, and is of `[u8; 8]`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with three DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first and third DES objects are encryptors
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ TDES, CBC_PKCS7 };
    /// 
    /// let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// let key2 = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    /// let key3 = [0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    /// let mut tdes = TDES::new_with_3_keys(key1, key2, key3);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8E 74 E3 51 84 23 B5 36 76 B6 82 D1 8B 7A A3 1F 87 D2 48 9A 75 BF 59 0D 93 6D 8D A7 86 4A CC 0F D8 0D E0 CD 0D F9 A8 B9 38 36 0C E7 24 73 3F 5F 4D 61 AB 92 D6 34 14 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_3_keys(key1: [u8; 8], key2: [u8; 8], key3: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1)
                            + DES::decryptor_with_key(key2)
                            + DES::encryptor_with_key(key3)
    }

    // pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with three DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first and third DES objects,
    ///   and is of `[u8; 8]`.
    /// - `key2` is a key for the second DES object, and is of `[u8; 8]`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with three DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first and third DES objects are encryptors
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ TDES, CBC_PKCS7 };
    /// 
    /// let mut tdes = TDES::new_with_keys_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "17 AE B5 A8 D2 77 21 0C 73 52 2F EB 5B 7C 9B 82 47 71 D2 7A F0 A9 F0 EA EC 0A D5 61 CB 63 86 33 8C 1F F2 F1 16 62 A0 55 22 9E 12 7A 91 88 D1 37 7B CB 43 32 19 0D AA B0 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    {
        let key: LongerUnion = LongerUnion::new_with(key);
        BigCryptor64::new() + DES::encryptor_with_key_u64(key.get_ulong_(0))
                            + DES::decryptor_with_key_u64(key.get_ulong_(1))
                            + DES::encryptor_with_key_u64(key.get_ulong_(0))
    }
}


/// struct that generates BigCryptor64 with two DES objects
pub struct DDES {}

impl DDES
{
    // pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with two DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first DES object, and is of `u64`.
    /// - `key2` is a key for the second DES object, and is of `u64`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with two DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set three small cryptors (components) with given two keys.
    /// - All the three small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first DES object is an encryptor.
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DDES, CBC_PKCS7 };
    /// 
    /// let mut ddes = DDES::new_with_2_keys_u64(0x_1234567890ABCDEF_u64, 0x_FEDCBA0987654321_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// ddes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B0 56 0C 03 A4 38 1F 70 C1 21 99 C6 23 06 05 30 DA 57 5F CD 17 FB 11 BF E1 05 92 B3 FD A8 70 15 24 3F A1 29 B9 D1 F6 7A 1D 58 86 BE 40 41 26 15 4A DA D3 EB 4E 84 85 60 ");
    /// 
    /// let mut recovered = String::new();
    /// ddes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
    }

    // pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with two DES objects.
    /// 
    /// # Arguments
    /// - `key1` is a key for the first DES object, and is of `[u8; 8]`.
    /// - `key2` is a key for the second DES object, and is of `[u8; 8]`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with two DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set two small cryptors (components) with given two keys.
    /// - All the two small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first DES object is encryptor.
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DDES, CBC_PKCS7 };
    /// 
    /// let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// let key2 = [0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21];
    /// let mut ddes = DDES::new_with_2_keys(key1, key2);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// ddes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "26 FB A9 70 99 13 83 84 63 A7 17 79 DD C6 1D D0 3D 37 12 07 B3 79 FE 5F E7 E2 6A 06 FF EC 43 26 CB 38 D6 A0 74 05 C0 E8 18 86 3F AF EA 54 AC 12 64 B7 20 C5 7F 36 02 2F ");
    /// 
    /// let mut recovered = String::new();
    /// ddes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1)
                            + DES::decryptor_with_key(key2)
    }

    // pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    /// Constructs a new object BigCryptor64 with two DES objects.
    /// 
    /// # Arguments
    /// - `key` is a key for all the two DES objects,
    ///   and is of `u128`.
    /// 
    /// # Output
    /// The new object `BigCryptor64` with two DES objects
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method set two small cryptors (components) with given two keys.
    /// - All the two small cryptors (components) should have the block
    ///   size 64-bit.
    /// - The first DES object is encryptor.
    /// - The second DES object is a decryptor.
    /// 
    /// # Example 1
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DDES, CBC_PKCS7 };
    /// 
    /// let mut tdes = DDES::new_with_keys_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "8E C3 47 07 38 6E 5F E1 BA 9C AF C1 41 B5 22 E3 55 22 DE 83 60 21 41 86 5D 94 06 1A 6C 54 32 F8 58 BE 43 23 FC 80 99 5B 1C 9D 4B 5D 1B E7 8B 9F 91 9D 83 1B C7 3D C0 55 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    {
        let key: LongerUnion = LongerUnion::new_with(key);
        BigCryptor64::new() + DES::encryptor_with_key_u64(key.get_ulong_(0))
                            + DES::decryptor_with_key_u64(key.get_ulong_(1))
    }
}
