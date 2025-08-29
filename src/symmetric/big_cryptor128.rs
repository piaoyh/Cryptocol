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
use crate::number::{ LongerUnion, SmallUInt };
use crate::symmetric::SmallCryptor;



/// A BigCryptor128 wrapper struct for cascading encryption/decryption
/// algorithms that has 128-bit block size
/// 
/// # Introduction
/// BigCryptor128 is mainly used to make cascade encryption/decryption
/// that has 128-bit block size.
///
/// # Quick Start
/// You have to import (use) the module `BigCryptor128` in order to cascade
/// encryption/decryption algorithms that has 128-bit block size
/// such as AES-128 cascade as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::BigCryptor128;
/// ```
/// 
/// You can instantiate the BigCryptor128 object with components (hereinafter,
/// referred to as small cryptor) that has `u128` keys as Example 2,
/// for example. In this case, you have to take endianness into account.
/// In little-endianness, 0x_1234567890ABCDEFFEDCBA0987654321_u128 is [0x21u8,
/// 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78,
/// 0x56, 0x34, 0x12] while the same 0x_1234567890ABCDEFFEDCBA0987654321_u128
/// is [0x12u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA,
/// 0x09, 0x87, 0x65, 0x43, 0x21] in big-endianness.
/// The instantiated object should be mutable.
/// 
/// # Example 2
/// ```
/// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
/// let mut _taes = BigCryptor128::new()
///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
/// 
/// ```
/// 
/// Also, you can instantiate the BigCryptor128 object with small cryptors
/// (component) that have `[u8; 16]` keys as shown in Example 3. In this case,
/// you don't have to take endianness into account. The instantiated object
/// should be mutable.
/// 
/// # Example 3
/// ```
/// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
/// let mut _taes = BigCryptor128::new()
///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// ```
/// 
/// You can instantiate the BigCryptor128 object without small cryptors
/// (components) that have key and set a `u128` key later as shown
/// in Example 4 or `[u8; 16]` keys later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
/// let mut taes = BigCryptor128::new();
/// let aes1 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
/// let aes2 = AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
/// let aes3 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
/// taes.push_small_cryptor(aes1);
/// taes.push_small_cryptor(aes2);
/// taes.push_small_cryptor(aes3);
/// ```
/// 
/// # Example 5
/// ```
/// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
/// let mut taes = BigCryptor128::new();
/// let aes1 = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// let aes2 = AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// let aes3 = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// taes.push_small_cryptor(aes1);
/// taes.push_small_cryptor(aes2);
/// taes.push_small_cryptor(aes3);
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
/// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
/// let mut taes = BigCryptor128::new()
///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// 
/// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
/// println!("IV =	{:#034X}", iv);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let mut cipher = Vec::<u8>::new();
/// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "1E 24 26 AD 13 7F 6F 6A CD 22 3A 4F A5 24 D8 E0 E3 6A B2 39 0D 82 2B 6E 7B D6 95 09 6D EF C2 7B 30 53 87 B7 9C D3 21 7C C0 85 11 74 28 7B 98 7B 9F 02 54 81 23 96 6D F5 A1 39 C8 A2 4B 20 76 7A ");
/// /// let mut recovered = String::new();
/// taes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
pub struct BigCryptor128
{
    block: LongerUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>,
    enc: fn (s: &mut Self, message: u128) -> u128,
    dec: fn (s: &mut Self, cipher: u128) -> u128,
}

impl BigCryptor128
{
    pub(super) const BLOCK_SIZE: usize = 16;
    const SUCCESS: u128 = !0;
    const FAILURE: u128 = 0;

    // pub fn new() -> Self
    /// Constructs a new object BigCryptor128.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has no small cryptors by default.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method does not set any small cryptor (component) by default.
    /// - You have to set as many small cryptors (components) as you want.
    /// - The small cryptors (components) should have the block size 128-bit.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = BigCryptor128::new();
    /// taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// taes.push_small_cryptor(AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.new)           
    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            block:          LongerUnion::zero(),
            smallcryptor:   Vec::new(),
            enc:            Self::encrypt_u128,
            dec:            Self::decrypt_u128,
        }
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N]) -> Self
    /// Constructs a new object BigCryptor128 with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// `smallcryptor` is the array of small cryptors (components)
    /// wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// 
    /// let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// let mut _taes = BigCryptor128::new_with_small_cryptor_array(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.new_with_small_cryptor_array)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>) -> Self
    /// Constructs a new object `BigCryptor128` with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// - `smallcryptor` is the `Vec` object of small cryptors (components)
    ///  wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// let cryptors: Vec<Box<dyn SmallCryptor<u128, 16>>> = vec![ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// let mut _taes = BigCryptor128::new_with_small_cryptor_vec(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.new_with_small_cryptor_vec)
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    // pub fn push_small_cryptor<S: SmallCryptor<u128, 16> + 'static>(&mut self, smallcryptor: S)
    /// Adds a small cryptor (component) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptor` is a small cryptors (components).
    /// 
    /// # Features
    /// This method automatically wraps the small cryptor by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// let aes1 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let aes2 = AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    /// let aes3 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// taes.push_small_cryptor(aes1);
    /// taes.push_small_cryptor(aes2);
    /// taes.push_small_cryptor(aes3);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.push_small_cryptor)
    pub fn push_small_cryptor<S: SmallCryptor<u128, 16> + 'static>(&mut self, smallcryptor: S)
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
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// taes.push_small_cryptor_array(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.push_small_cryptor_array)
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N])
    {
        for val in smallcryptor
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec<const N: usize>(&mut self, smallcryptor: [SmallCryptor; N])
    /// Adds small cryptors (components) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptors` is a `Vec` object of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the Vec object of the small cryptors
    /// should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// let cryptors: Vec<Box<dyn SmallCryptor<u128, 16>>> = vec![ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// taes.push_small_cryptor_vec(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.push_small_cryptor_vec)
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>)
    {
        self.smallcryptor = smallcryptor;
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor128.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_inverse();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_B881F06147B26243D0742CAA82602E97_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.turn_inverse)
    #[inline]
    pub fn turn_inverse(&mut self)
    {
        (self.enc, self.dec) = (self.dec, self.enc);
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 to encryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_encryptor();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_1E561632CF3EDD44E8955A26ABA0AF7E_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.turn_encryptor)
    pub fn turn_encryptor(&mut self)
    {
        self.enc = Self::encrypt_u128;
        self.dec = Self::decrypt_u128;
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 to encryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_decryptor();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_B881F06147B26243D0742CAA82602E97_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.turn_decryptor)
    pub fn turn_decryptor(&mut self)
    {
        self.enc = Self::decrypt_u128;
        self.dec = Self::encrypt_u128;
    }

    // pub fn encrypt_u128(&mut self, message: u128) -> u128
    /// Encrypts a 128-bit data.
    /// 
    /// # Arguments
    /// `message` is of `u128`-type and the plaintext to be encrypted.
    /// 
    /// # Output
    /// This method returns the encrypted data of `u128`-type from `message`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("M = {:#034X}", message);
    /// let cipher = taes.encrypt_u128(message);
    /// println!("C = {:#034X}", cipher);
    /// assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.encrypt_u128)
    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        self.set_block(message);
        self.encrypt_block();
        self.block.get()
    }

    // pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    /// Decrypts a 128-bit data.
    /// 
    /// # Arguments
    /// `cioher` is of `u128`-type and the ciphertext to be decrypted.
    /// 
    /// # Output
    /// This method returns the decrypted data of `u128`-type from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("M = {:#034X}", message);
    /// let cipher = taes.encrypt_u128(message);
    /// println!("C = {:#034X}", cipher);
    /// assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// 
    /// let recovered = taes.decrypt_u128(cipher);
    /// println!("B = {:#034X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.decrypt_u128)
    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.block.get()
    }

    #[inline]
    pub(super) fn _encrypt(&mut self, message: u128) -> u128
    {
        (self.enc)(self, message)
    }

    #[inline]
    pub(super) fn _decrypt(&mut self, cipher: u128) -> u128
    {
        (self.dec)(self, cipher)
    }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    /// Encrypts an array of 128-bit data.
    /// 
    /// # Arguments
    /// - `message` is of an array of `u128`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211_u128];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#034X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u128; 3];
    /// taes.encrypt_array_u128(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#034X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    /// assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.encrypt_array_u128)
    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        for i in 0..N
            { cipher[i] = self.encrypt_u128(message[i]); }
    }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    /// Decrypts an array of 128-bit data.
    /// 
    /// # Arguments
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u128`-type and the plaintext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#034X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u128; 3];
    /// taes.encrypt_array_u128(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#034X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    /// assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);
    /// 
    /// let mut recovered = [0_u128; 3];
    /// taes.decrypt_array_u128(&cipher, &mut recovered);
    /// print!("B = ");
    /// for r in recovered.clone()
    ///     { print!("{:#034X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered[1], 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered[2], 0x_FFEEDDCCBBAA00998877665544332211_u128);
    /// assert_eq!(recovered[0], message[0]);
    /// assert_eq!(recovered[1], message[1]);
    /// assert_eq!(recovered[2], message[2]);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.decrypt_array_u128)
    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    {
        for i in 0..N
            { message[i] = self.decrypt_u128(cipher[i]); }
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
    /// - If the ciphertext is 16 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    /// 
    /// # Example 1 for Normal case for Successful case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let success = taes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.is_successful)
    #[inline]
    pub fn is_successful(&self) -> bool
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
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let failure = taes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor128_basic/struct.BigCryptor128.html#method.is_failed)
    #[inline]
    pub fn is_failed(&self) -> bool
    {
        self.block.get() == Self::FAILURE
    }

    // pub fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```compile_fail
    /// use cryptocol::symmetric::AES;
    /// let mut a_aes = AES::new_with_key_u64(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(a_des.is_successful(), false);
    /// 
    /// a_aes.set_successful();
    /// assert_eq!(a_aes.is_successful(), true);
    /// ```
    #[inline]
    pub fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    // pub fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```compile_fail
    /// use cryptocol::symmetric::AES;
    /// let mut a_aes = AES::new_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// a_aes.encrypt_u128(0x1234567890ABCDEF_u64);
    /// assert_eq!(a_aes.is_failed(), false);
    /// 
    /// a_aes.set_failed();
    /// assert_eq!(a_aes.is_failed(), true);
    /// ```
    #[inline]
    pub fn set_failed(&mut self)
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

    #[inline] fn get_block(&self) -> u128            { self.block.get() }
    #[inline] fn set_block(&mut self, block: u128)   { self.block.set(block); }
}
