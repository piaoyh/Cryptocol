// Copyright 2025, 2026 PARK Youngho.
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



/// des.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_basic.rs.
#[allow(non_camel_case_types)]
pub struct DES_Generic
{
    // Dummy struct for documentation
}

/// des.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl DES_Generic
{
    // pub fn new() -> Self
    /// Constructs a new object DES_Generic.
    ///
    /// # Features
    /// - In order to encrypt date, object should be instantiated mutable.
    /// - This method sets the key to be [0, 0, 0, 0, 0, 0, 0, 0].
    /// - Do not use this default key [0, 0, 0, 0, 0, 0, 0, 0]
    ///   because it is known as one of the weak keys.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new();   // The default key is 0x0000000000000000 which is a weak key.
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x1E32B46B44C69201_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);  // So, you can't use the default key!!!
    /// ```
    ///
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::DES;
    /// let des = DES::new();
    /// // It cannot be compiled!
    /// des.encrypt_u64(0x1E32B46B44C69201_u64);
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn box_new() -> Box<Self>
    /// Constructs a new object DES_Generic wrapped by Box.
    ///
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method sets the key to be [0_u8, 0, 0, 0, 0, 0, 0, 0].
    /// - Do not use this default key [0_u8, 0, 0, 0, 0, 0, 0, 0]
    ///   because it is known as one of the weak keys.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::box_new();   // The default key is 0x0000000000000000 which is a weak key.
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x1E32B46B44C69201_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);  // So, you can't use the default key!!!
    /// ```
    ///
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::DES;
    /// let des = DES::box_new();
    /// // It cannot be compiled!
    /// des.encrypt_u64(0x1E32B46B44C69201_u64);
    /// ```
    #[inline]
    pub fn box_new() -> Box<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_key(key: [u8; 8]) -> Self
    /// Constructs a new object DES_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// This method sets the key to be the given argument `key`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// let mut des = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    /// assert_ne!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for Weak key case for [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// ```
    /// // The key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] is the same key as the key
    /// // [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01] because of parity bits.
    ///
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// let mut des2 = DES::new_with_key([0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// // and [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]!!!
    /// ```
    ///
    /// # Example 3 for Weak key case for [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    /// ```
    /// // The key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] is the same key as the key
    /// // [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE] because of parity bits.
    ///
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    /// let mut des2 = DES::new_with_key([0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    /// // and [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]!!!
    /// ```
    ///
    /// # Example 4 for Weak key case for [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    /// ```
    /// // The key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1] is the same key as the key
    /// // [0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0] because of parity bits.
    ///
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]);
    /// let mut des2 = DES::new_with_key([0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    /// // and [0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1]!!!
    /// ```
    ///
    /// # Example 5 for Weak key case for [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    /// ```
    /// // The key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E] is the same key as the key
    /// // [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F] because of parity bits.
    ///
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]);
    /// let mut des2 = DES::new_with_key([0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    /// // and [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]!!!
    /// ```
    ///
    /// # Example 6 for Semi-Weak key case for [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E] and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]);
    /// let mut des2 = DES::new_with_key([0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x063A6E55466423D2_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]
    /// // and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]!!!
    /// ```
    ///
    /// # Example 7 for Semi-Weak key case for [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1] and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]);
    /// let mut des2 = DES::new_with_key([0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]
    /// // and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]!!!
    /// ```
    ///
    /// # Example 8 for Semi-Weak key case for [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE] and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // Semi-Weak key case 3 for
    /// let mut des1 = DES::new_with_key([0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]);
    /// let mut des2 = DES::new_with_key([0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x7EE95658A653960D_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]
    /// // and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]!!!
    /// ```
    ///
    /// # Example 9 for Semi-Weak key case for [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1] and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // Semi-Weak key case 4 for
    /// let mut des1 = DES::new_with_key([0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]);
    /// let mut des2 = DES::new_with_key([0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]
    /// // and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]!!!
    /// ```
    ///
    /// # Example 10 for Semi-Weak key case for [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE] and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key([0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]);
    /// let mut des2 = DES::new_with_key([0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]
    /// // and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]!!!
    /// ```
    ///
    /// # Example 11 for Semi-Weak key case for [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE] and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // Semi-Weak key case 6 for
    /// let mut des1 = DES::new_with_key([0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]);
    /// let mut des2 = DES::new_with_key([0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x27C83AAE29571889_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xDE76DF630C033919_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]
    /// // and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]!!!
    /// ```
    ///
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::DES;
    /// let des = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// // It cannot be compiled!
    /// des.encrypt_u64(0x1E32B46B44C69201_u64);
    /// ```
    pub fn new_with_key(_key: [u8; 8]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_key_u64(key: u64) -> Self
    /// Constructs a new object DES_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is of `u64`.
    /// - It should be in the same endianness of machine. For example,
    ///   if a key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF],
    ///   the key in `u64` is 0x_1234567890ABCDEF_u64 for big-endian machine,
    ///   and the key in `u64` is 0x_EFCDAB9078563412_u64 for little-endian
    ///   machine.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x_0000_0000_0000_0000_u4, 0x_0101_0101_0101_0101_u64,
    ///   0x_0000_0000_0000_0001_u64, 0x_0000_0000_0000_0100_u64,
    ///   0x_0100_0010_0000_0001_u64, etc. are all the same keys.
    ///   Each key has 255 different equivalent keys in DES.
    ///
    /// # Features
    /// This method sets the key to be the given argument `key`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new_with_key_u64(0xEFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    /// assert_ne!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for Weak key case for 0x0000000000000000
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0x0000000000000000 is the same key as the key 0x0101010101010101 because of parity bits.
    /// let mut des1 = DES::new_with_key_u64(0x0000000000000000);
    /// let mut des2 = DES::new_with_key_u64(0x0101010101010101);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0x0000000000000000 and 0x0101010101010101!!!
    /// ```
    ///
    /// # Example 3 for Weak key case for 0xFFFFFFFFFFFFFFFF
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0xFFFFFFFFFFFFFFFF is the same key as the key 0xFEFEFEFEFEFEFEFE because of parity bits.
    /// let mut des1 = DES::new_with_key_u64(0xFFFFFFFFFFFFFFFF);
    /// let mut des2 = DES::new_with_key_u64(0xFEFEFEFEFEFEFEFE);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0xFFFFFFFFFFFFFFFF and 0xFEFEFEFEFEFEFEFE!!!
    /// ```
    ///
    /// # Example 4 for Weak key case for 0xF1F1F1F1E0E0E0E0 in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0xF1F1F1F1E0E0E0E0 is the same key as the key 0xF0F0F0F0E1E1E1E1 because of parity bits.
    /// let mut des1 = DES::new_with_key_u64(0xF1F1F1F1E0E0E0E0);
    /// let mut des2 = DES::new_with_key_u64(0xF0F0F0F0E1E1E1E1);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0xF1F1F1F1E0E0E0E0 and 0xF0F0F0F0E1E1E1E1!!!
    /// ```
    ///
    /// # Example 5 for Weak key case for 0x0E0E0E0E1F1F1F1F in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0x0E0E0E0E1F1F1F1F is the same key as the key 0x0F0F0F0F1E1E1E1E because of parity bits.
    /// let mut des1 = DES::new_with_key_u64(0x0E0E0E0E1F1F1F1F);
    /// let mut des2 = DES::new_with_key_u64(0x0F0F0F0F1E1E1E1E);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0x0E0E0E0E1F1F1F1F and 0x0F0F0F0F1E1E1E1E!!!
    /// ```
    ///
    /// # Example 6 for Weak key case for 0x0E010E011F011F01 and 0x010E010E011F011F in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0x0E010E011F011F01);
    /// let mut des2 = DES::new_with_key_u64(0x010E010E011F011F);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x063A6E55466423D2_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0x0E010E011F011F01 and 0x010E010E011F011F!!!
    /// ```
    ///
    /// # Example 7 for Weak key case for 0xF101F101E001E001 and 0x01F101F101E001E0 in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0xF101F101E001E001);
    /// let mut des2 = DES::new_with_key_u64(0x01F101F101E001E0);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xF101F101E001E001 and 0x01F101F101E001E0!!!
    /// ```
    ///
    /// # Example 8 for Weak key case for 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0xFE01FE01FE01FE01);
    /// let mut des2 = DES::new_with_key_u64(0x01FE01FE01FE01FE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x7EE95658A653960D_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE!!!
    /// ```
    ///
    /// # Example 9 for Weak key case for 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0 in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0xF10EF10EE01FE01F);
    /// let mut des2 = DES::new_with_key_u64(0x0EF10EF11FE01FE0);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0!!!
    /// ```
    ///
    /// # Example 10 for Weak key case for 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0xFE0EFE0EFE1FFE1F);
    /// let mut des2 = DES::new_with_key_u64(0x0EFE0EFE1FFE1FFE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE!!!
    /// ```
    ///
    /// # Example 11 for Weak key case for 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new_with_key_u64(0xFEF1FEF1FEE0FEE0);
    /// let mut des2 = DES::new_with_key_u64(0xF1FEF1FEE0FEE0FE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x27C83AAE29571889_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xDE76DF630C033919_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE!!!
    /// ```
    pub fn new_with_key_u64(_key: u64) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encryptor_with_key(key: [u8; 8]) -> Self
    /// Constructs a new object DES_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor64 incluing NDES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// - You won't use this method unless you use NDES for such as Triple DES.
    /// - This method sets the key to be the given argument `key`.
    /// - This method constructs the encryptor component of NDES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
    ///         = [ Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(DES::decryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    ///
    /// let mut tdes = DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
    ///                 - DES::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
    ///                 + DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    #[inline]
    pub fn encryptor_with_key(_key: [u8; 8]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encryptor_with_key_u64(key: u64) -> Self
    /// Constructs a new object DES_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor64 incluing NDES.
    ///
    /// # Arguments
    /// - The argument `key` is an unsigned integer that is of `u64`-type.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x0000000000000000_u64, 0x0101010101010101_u64,
    ///   0x0000000000000001_u64, 0x0000000000000100_u64, 0x0100001000000001,
    ///   etc. are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// - You won't use this method unless you use NDES for such as Triple DES.
    /// - This method sets the key to be the given argument `key`.
    /// - This method constructs the encryptor component of NDES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(
    ///             [Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///             Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///             Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64))]
    /// );
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    ///
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 - DES::encryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    #[inline]
    pub fn encryptor_with_key_u64(key: u64) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decryptor_with_key(key: [u8; 8]) -> Self
    /// Constructs a new object DES_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor64 incluing NDES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// - You won't use this method unless you use NDES for such as Triple DES.
    /// - This method sets the key to be the given argument `key`.
    /// - This method constructs the encryptor component of NDES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
    ///         = [ Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(DES::decryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    ///
    /// let mut tdes = BigCryptor64::new()
    ///                 - DES::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
    ///                 - DES::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
    ///                 - DES::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    #[inline]
    pub fn decryptor_with_key(key: [u8; 8]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decryptor_with_key_u64(key: u64) -> Self
    /// Constructs a new object DES_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor64 incluing NDES.
    ///
    /// # Arguments
    /// - The argument `key` is an unsigned integer that is of `u64`-type.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x0000000000000000_u64, 0x0101010101010101_u64,
    ///   0x0000000000000001_u64, 0x0000000000000100_u64, 0x0100001000000001,
    ///   etc. are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// - You won't use this method unless you use NDES for such as Triple DES.
    /// - This method sets the key to be the given argument `key`.
    /// - This method constructs the decryptor component of NDES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(
    ///                 [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                 Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                 Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ] );
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    ///
    /// let mut tdes = BigCryptor64::new()
    ///                 - DES::decryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 - DES::decryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    #[inline]
    pub fn decryptor_with_key_u64(key: u64) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_key(&mut self) -> [u8; 8]
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of array of `u8`.
    ///
    /// # Example
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new();
    /// des.set_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = des.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:X02#} ", k); }
    /// assert_eq!(key, [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// ```
    pub fn get_key(&mut self) -> [u8; 8]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_key_u64(&self) -> u64
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of `u64`.
    ///
    /// # Example
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new();
    /// des.set_key_u64(0xEFCDAB9078563412);
    /// let key = des.get_key_u64();
    /// println!("Key = {}", key);
    /// assert_eq!(key, 0xEFCDAB9078563412_u64);
    /// ```
    #[inline]
    pub fn get_key_u64(&self) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_key(&mut self, key: [u8; 8])
    /// Sets the key.
    ///
    /// # Arguments
    /// - The argument `key` is the array of `u8` that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES.
    ///
    /// # Features
    /// This method sets the key to be the given argument `key`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new();
    /// des.set_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    /// assert_ne!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for Weak key case for [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] is the same key as the key
    /// // [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01] because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// des2.set_key([0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// // and [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]!!!
    /// ```
    ///
    /// # Example 3 for Weak key case for [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] is the same key as the key
    /// // [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE] because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    /// des2.set_key([0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    /// // and [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]!!!
    /// ```
    ///
    /// # Example 4 for Weak key case for [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key [0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0] is the same key as the key
    /// // [0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0] because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]);
    /// des2.set_key([0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    /// // and [0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1]!!!
    /// ```
    ///
    /// # Example 5 for Weak key case for [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E] is the same key as the key
    /// // [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F] because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]);
    /// des2.set_key([0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    /// // and [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]!!!
    /// ```
    ///
    /// # Example 5 for Semi-Weak key case for [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E] and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]);
    /// des2.set_key([0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x063A6E55466423D2_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]
    /// // and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]!!!
    /// ```
    ///
    /// # Example 6 for Semi-Weak key case for [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1] and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]);
    /// des2.set_key([0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]
    /// // and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]!!!
    /// ```
    ///
    /// # Example 7 for Semi-Weak key case for [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE] and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]);
    /// des2.set_key([0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x7EE95658A653960D_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]
    /// // and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]!!!
    /// ```
    ///
    /// # Example 8 for Semi-Weak key case for [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1] and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]);
    /// des2.set_key([0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]
    /// // and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]!!!
    /// ```
    ///
    /// # Example 9 for Semi-Weak key case for [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE] and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]);
    /// des2.set_key([0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]
    /// // and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]!!!
    /// ```
    ///
    /// # Example 10 for Semi-Weak key case for [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE] and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key([0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]);
    /// des2.set_key([0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x27C83AAE29571889_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xDE76DF630C033919_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]
    /// // and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]!!!
    /// ```
    pub fn set_key(&mut self, key: [u8; 8])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_key_u64(&mut self, key: u64)
    /// Sets the key.
    ///
    /// # Arguments
    /// - The argument `key` is of `u64`.
    /// - It should be in the same endianness of machine. For example,
    ///   if a key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF],
    ///   the key in `u64` is 0x_1234567890ABCDEF_u64 for big-endian machine,
    ///   and the key in `u64` is 0x_EFCDAB9078563412_u64 for little-endian
    ///   machine.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x_0000_0000_0000_0000_u4, 0x_0101_0101_0101_0101_u64,
    ///   0x_0000_0000_0000_0001_u64, 0x_0000_0000_0000_0100_u64,
    ///   0x_0100_0010_0000_0001_u64, etc. are all the same keys.
    ///   Each key has 255 different equivalent keys in DES.
    ///
    /// # Features
    /// This method sets the key to be the given argument `key`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des = DES::new();
    /// des.set_key_u64(0xEFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);
    ///
    /// let cipher_cipher_text = des.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    /// assert_ne!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for Weak key case for 0x0000000000000000
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0x0000000000000000 is the same key as the key 0x0101010101010101 because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0x0000000000000000);
    /// des2.set_key_u64(0x0101010101010101);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0x0000000000000000 and 0x0101010101010101!!!
    /// ```
    ///
    /// # Example 3 for Weak key case for 0xFFFFFFFFFFFFFFFF
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0xFFFFFFFFFFFFFFFF is the same key as the key 0xFEFEFEFEFEFEFEFE because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xFFFFFFFFFFFFFFFF);
    /// des2.set_key_u64(0xFEFEFEFEFEFEFEFE);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0xFFFFFFFFFFFFFFFF and 0xFEFEFEFEFEFEFEFE!!!
    /// ```
    ///
    /// # Example 4 for Weak key case for 0xF1F1F1F1E0E0E0E0 in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0xF1F1F1F1E0E0E0E0 is the same key as the key 0xF0F0F0F0E1E1E1E1 because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xF1F1F1F1E0E0E0E0);
    /// des2.set_key_u64(0xF0F0F0F0E1E1E1E1);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0xF1F1F1F1E0E0E0E0 and 0xF0F0F0F0E1E1E1E1!!!
    /// ```
    ///
    /// # Example 5 for Weak key case for 0x0E0E0E0E1F1F1F1F in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// // The key 0x0E0E0E0E1F1F1F1F is the same key as the key 0x0F0F0F0F1E1E1E1E because of parity bits.
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0x0E0E0E0E1F1F1F1F);
    /// des2.set_key_u64(0x0F0F0F0F1E1E1E1E);
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext1 = des1.encrypt_u64(plaintext);
    /// let ciphertext2 = des2.encrypt_u64(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#018X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#018X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    /// let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    /// println!("Cipher-ciphertext1:\t{:#018X}\n", cipher_cipher_text1);
    /// println!("Cipher-ciphertext2:\t{:#018X}\n", cipher_cipher_text2);
    /// assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text1, plaintext);
    /// assert_eq!(cipher_cipher_text2, plaintext);
    /// // So, you can't use the weak key 0x0E0E0E0E1F1F1F1F and 0x0F0F0F0F1E1E1E1E!!!
    /// ```
    ///
    /// # Example 6 for Semi-Weak key case for 0x0E010E011F011F01 and 0x010E010E011F011F in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0x0E010E011F011F01);
    /// des2.set_key_u64(0x010E010E011F011F);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x063A6E55466423D2_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0x0E010E011F011F01 and 0x010E010E011F011F!!!
    /// ```
    ///
    /// # Example 7 for Semi-Weak key case for 0x0E010E011F011F01 and 0x010E010E011F011F in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xF101F101E001E001);
    /// des2.set_key_u64(0x01F101F101E001E0);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xF101F101E001E001 and 0x01F101F101E001E0!!!
    /// ```
    ///
    /// # Example 8 for Semi-Weak key case for 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xFE01FE01FE01FE01);
    /// des2.set_key_u64(0x01FE01FE01FE01FE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x7EE95658A653960D_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE!!!
    /// ```
    ///
    /// # Example 9 for Semi-Weak key case for 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0 in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xF10EF10EE01FE01F);
    /// des2.set_key_u64(0x0EF10EF11FE01FE0);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0!!!
    /// ```
    ///
    /// # Example 10 for Semi-Weak key case for 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xFE0EFE0EFE1FFE1F);
    /// des2.set_key_u64(0x0EFE0EFE1FFE1FFE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE!!!
    /// ```
    ///
    /// # Example 11 for Semi-Weak key case for 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE in little-endianness
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let mut des1 = DES::new();
    /// let mut des2 = DES::new();
    /// des1.set_key_u64(0xFEF1FEF1FEE0FEE0);
    /// des2.set_key_u64(0xF1FEF1FEE0FEE0FE);
    ///
    /// let plaintext = 0x1234567890ABCDEF_u64;
    /// let ciphertext = des1.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x27C83AAE29571889_u64);
    ///
    /// let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    ///
    /// let ciphertext = des2.encrypt_u64(plaintext);
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0xDE76DF630C033919_u64);
    ///
    /// let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}\n", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// // So, you can't use the semi-weak keys 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE!!!
    /// ```
    pub fn set_key_u64(&mut self, key: u64)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor64.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor64 or NDES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor64
    ///   or NDES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@DES_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u64](struct@DES_Generic#method.encryptor_with_key_u64),
    ///   [decryptor_with_key](struct@DES_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u64](struct@DES_Generic#method.decryptor_with_key_u64).
    /// - If it is constructed as encryptor for BigCryptor64 or NDES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for BigCryptor64 or NDES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
    ///             = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                 Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                 Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// keys[1].turn_inverse();
    /// 
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let mut des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    /// let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// des2.turn_inverse();
    /// 
    /// let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    pub fn turn_inverse(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor64 or NDES to encryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor64 or NDES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor64
    ///   or NDES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@DES_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u64](struct@DES_Generic#method.encryptor_with_key_u64),
    ///   [decryptor_with_key](struct@DES_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u64](struct@DES_Generic#method.decryptor_with_key_u64).
    /// - If it is constructed as encryptor for BigCryptor64 or NDES,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for BigCryptor64 or NDES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
    ///         = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///             Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///             Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// keys[0].turn_encryptor();
    /// 
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let mut des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    /// let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// des1.turn_encryptor();
    /// 
    /// let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    pub fn turn_encryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_decryptor(&mut self)
    /// Changes its role in BigCryptor64 or NDES to decryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor64 or NDES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor64
    ///   or NDES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@DES_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u64](struct@DES_Generic#method.encryptor_with_key_u64),
    ///   [decryptor_with_key](struct@DES_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u64](struct@DES_Generic#method.decryptor_with_key_u64).
    /// - If it is constructed as encryptor for BigCryptor64 or NDES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for BigCryptor64 or NDES,
    ///   it will not be changed at all.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
    ///             = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                 Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                 Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// keys[1].turn_decryptor();
    /// 
    /// let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };
    /// 
    /// let des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let mut des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    /// let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// des2.turn_decryptor();
    /// 
    /// let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_1234567890ABCDEF_u64;
    /// let ciphertext = tdes.encrypt_u64(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#018X}", plaintext);
    /// println!("Ciphertext:\t\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#018X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    pub fn turn_decryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
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
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des.encrypt_u64(message);
    /// println!("C_u64 (16 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    /// ```
    ///
    /// # Example 2 for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let cipher = b_des.encrypt_u64(message);
    /// println!("C_u64 (128 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    /// ```
    ///
    /// # Example 3 for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K1 =\t{:#016x}", key1);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let cipher1 = c_des.encrypt_u64(message);
    /// let cipher2 = d_des.encrypt_u64(message);
    /// println!("C_u64 (0 rounds) =\t{:#018X}", cipher1);
    /// assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);
    ///
    /// println!("D_u64 (0 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1, cipher2);
    /// ```
    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
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
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des.encrypt_u64(message);
    /// println!("C_u64 (16 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    ///
    /// let recovered = a_des.decrypt_u64(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#018X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let cipher = b_des.encrypt_u64(message);
    /// println!("C_u64 (128 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    ///
    /// let recovered = b_des.decrypt_u64(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#018X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::{ DES, DES_Expanded };
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let cipher1 = c_des.encrypt_u64(message);
    /// let cipher2 = d_des.encrypt_u64(message);
    /// println!("C_u64 (0 rounds) =\t{:#018X}", cipher1);
    /// assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);
    ///
    /// println!("D_u64 (0 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1, cipher2);
    ///
    /// let recovered1 = c_des.decrypt_u64(cipher1);
    /// let recovered2 = d_des.decrypt_u64(cipher2);
    /// println!("B1_u64 (0 rounds) =\t{:#018X}", recovered1);
    /// println!("B2_u64 (0 rounds) =\t{:#018X}", recovered2);
    /// assert_eq!(recovered1, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn _encrypt(&mut self, message: u64) -> u64
    /// Encrypts a 64-bit data when NDES encrypting if the object was
    /// constructed as encryptor for NDES such as Triple DES, and decrypts a
    /// 64-bit data when NDES encrypting if the object was constructed as
    /// decryptor for NDES such as Triple DES.
    ///
    /// # Arguments
    /// `message` is of `u64`-type and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data of `u64`-type from `message`.
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des._encrypt(message);
    /// println!("C_u64 (16 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let cipher = b_des._encrypt(message);
    /// println!("C_u64 (128 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K1 =\t{:#016x}", key1);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let cipher1 = c_des._encrypt(message);
    /// let cipher2 = d_des._encrypt(message);
    /// println!("C_u64 (0 rounds) =\t{:#018X}", cipher1);
    /// assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);
    ///
    /// println!("D_u64 (0 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1, cipher2);
    /// ```
    pub fn _encrypt(&mut self, message: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn _decrypt(&mut self, cipher: u64) -> u64
    /// Decrypts a 64-bit data when NDES decrypting if the object was
    /// constructed as encryptor for NDES such as Triple DES, and encrypts a
    /// 64-bit data when NDES decrypting if the object was constructed as
    /// decryptor for NDES such as Triple DES.
    ///
    /// # Arguments
    /// `cipher` is of `u64`-type and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data of `u64`-type from `cipher`.
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des._encrypt(message);
    /// println!("C_u64 (16 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    ///
    /// let recovered = a_des._decrypt(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#018X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let cipher = b_des._encrypt(message);
    /// println!("C_u64 (128 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    ///
    /// let recovered = b_des._decrypt(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#018X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#018X}", message);
    ///
    /// let cipher1 = c_des._encrypt(message);
    /// let cipher2 = d_des._encrypt(message);
    /// println!("C_u64 (0 rounds) =\t{:#018X}", cipher1);
    /// assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);
    ///
    /// println!("D_u64 (0 rounds) =\t{:#018X}", cipher);
    /// assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1, cipher2);
    ///
    /// let recovered1 = c_des._decrypt(cipher1);
    /// let recovered2 = d_des._decrypt(cipher2);
    /// println!("B1_u64 (0 rounds) =\t{:#018X}", recovered1);
    /// println!("B2_u64 (0 rounds) =\t{:#018X}", recovered2);
    /// assert_eq!(recovered1, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    pub fn _decrypt(&mut self, cipher: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_u64<const N: usize>(&mut self, message: &[u64; N], cipher: &mut [u64; N])
    /// Encrypts an array of 64-bit data.
    ///
    /// # Arguments
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be stored.
    ///
    /// # Features
    /// This method encrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic Code Book) mode.
    ///
    /// # Counterpart methods
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](../../traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](../../traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](../../traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](../../traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](../../traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](../../traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to encrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](../../traits_cfb/trait.CFB.html)
    /// - If you need to encrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](../../traits_ofb/trait.OFB.html)
    /// - If you need to encrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](../../traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](../../traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](../../traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](../../traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](../../traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](../../traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](../../traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](../../traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](../../traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](../../traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    /// ```
    ///
    /// # Example 2 for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let mut cipher = [0; 3];
    /// b_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_21F25F81CE4D4AA3_u64);
    /// assert_eq!(cipher[1], 0x_352F391A1482A504_u64);
    /// assert_eq!(cipher[2], 0x_F793546957AFDE50_u64);
    /// ```
    ///
    /// # Example 3 for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    ///
    /// let mut cipher1 = [0; 3];
    /// let mut cipher2 = [0; 3];
    /// c_des.encrypt_array_u64(&message, &mut cipher1);
    /// d_des.encrypt_array_u64(&message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher1[0], 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1[1], 0x_DFCE5760B4A93821_u64);
    /// assert_eq!(cipher1[2], 0x_FDEC75064B9A8312_u64);
    /// assert_eq!(cipher2[0], 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher2[1], 0x_DFCE5760B4A93821_u64);
    /// assert_eq!(cipher2[2], 0x_FDEC75064B9A8312_u64);
    /// assert_eq!(cipher1[0], cipher2[0]);
    /// assert_eq!(cipher1[1], cipher2[1]);
    /// assert_eq!(cipher1[2], cipher2[2]);
    /// ```
    pub fn encrypt_array_u64<const N: usize>(&mut self, message: &[u64; N], cipher: &mut [u64; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_u64<const N: usize>(&mut self, cipher: &[u64; N], message: &mut [u64; N])
    /// Decrypts an array of 64-bit data.
    ///
    /// # Arguments
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u64`-type and the plaintext to be stored.
    ///
    /// # Features
    /// This method decrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic Code Book) mode.
    ///
    /// # Counterpart methods
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](../../traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](../../traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](../../traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](../../traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](../../traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](../../traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to decrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](../../traits_cfb/trait.CFB.html)
    /// - If you need to decrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](../../traits_ofb/trait.OFB.html)
    /// - If you need to decrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](../../traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](../../traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](../../traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](../../traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](../../traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](../../traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](../../traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](../../traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](../../traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](../../traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    ///
    /// let mut recovered = [0; 3];
    /// a_des.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B (16 rounds) =\t");
    /// for r in recovered
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);
    /// ```
    ///
    /// # Example 2 for 128 rounds
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let mut cipher = [0; 3];
    /// b_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_21F25F81CE4D4AA3_u64);
    /// assert_eq!(cipher[1], 0x_352F391A1482A504_u64);
    /// assert_eq!(cipher[2], 0x_F793546957AFDE50_u64);
    ///
    /// let mut recovered = [0; 3];
    /// b_des.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B (128 rounds) =\t");
    /// for r in recovered
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);
    /// ```
    ///
    /// # Example 3 for 0 rounds which means that key is meaningless
    /// ```
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#018X} ", m); }
    /// println!();
    ///
    /// let mut cipher1 = [0; 3];
    /// let mut cipher2 = [0; 3];
    /// c_des.encrypt_array_u64(&message, &mut cipher1);
    /// d_des.encrypt_array_u64(&message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher1[0], 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher1[1], 0x_DFCE5760B4A93821_u64);
    /// assert_eq!(cipher1[2], 0x_FDEC75064B9A8312_u64);
    /// assert_eq!(cipher2[0], 0x_2138A9B46057CEDF_u64);
    /// assert_eq!(cipher2[1], 0x_DFCE5760B4A93821_u64);
    /// assert_eq!(cipher2[2], 0x_FDEC75064B9A8312_u64);
    /// assert_eq!(cipher1[0], cipher2[0]);
    /// assert_eq!(cipher1[1], cipher2[1]);
    /// assert_eq!(cipher1[2], cipher2[2]);
    ///
    /// let mut recovered1 = [0; 3];
    /// let mut recovered2 = [0; 3];
    /// c_des.decrypt_array_u64(&cipher1, &mut recovered1);
    /// d_des.decrypt_array_u64(&cipher2, &mut recovered2);
    /// print!("B1 (0 rounds) =\t");
    /// for r in recovered1
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// print!("B2 (0 rounds) =\t");
    /// for r in recovered2
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// assert_eq!(recovered1[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered1[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered1[2], 0x_FEDCBA0987654321_u64);
    /// assert_eq!(recovered2[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered2[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered2[2], 0x_FEDCBA0987654321_u64);
    /// assert_eq!(recovered1[0], recovered2[0]);
    /// assert_eq!(recovered1[1], recovered2[1]);
    /// assert_eq!(recovered1[2], recovered2[2]);
    /// ```
    pub fn decrypt_array_u64<const N: usize>(&mut self, cipher: &[u64; N], message: &mut [u64; N])
    {
        unimplemented!(); // Dummy code for documentation
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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let success = a_des.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    ///
    /// # Example 2 for Normal case for the original message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let cipher = [0x41u8, 0x7F, 0x89, 0x79, 0x08, 0xCD, 0xA1, 0x4C];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_array_into_array(&cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_des.is_successful();
    /// assert_eq!(success, true);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 3 for Failed case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CFB };
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 40];
    /// let len = a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_des.is_successful();
    /// assert_eq!(success, false);
    /// ```
    ///
    /// # Example 4 for Failed case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CFB };
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 40];
    /// let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_des.is_successful();
    /// assert_eq!(success, false);
    /// ```
    #[inline]
    pub fn is_successful(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_des.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let failure = a_des.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    ///
    /// # Example 2 for Normal case for the original message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    ///
    /// let cipher = [0x41u8, 0x7F, 0x89, 0x79, 0x08, 0xCD, 0xA1, 0x4C];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_array_with_padding_pkcs7_into_array(&cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_des.is_failed();
    /// assert_eq!(failure, false);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 3 for Failed case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 40];
    /// let len = a_des.encrypt_cfb_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_des.is_failed();
    /// assert_eq!(failure, true);
    /// ```
    ///
    /// # Example 4 for Failed case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// let mut recovered = [0u8; 40];
    /// let len = a_des.decrypt_array_cfb_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_des.is_failed();
    /// assert_eq!(failure, true);
    /// ```
    #[inline]
    pub fn is_failed(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub(super) fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    ///
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for NDES.
    ///
    /// # Example
    /// ```
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
        unimplemented!(); // Dummy code for documentation
    }

    // pub(super) fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    ///
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for NDES.
    ///
    /// # Example
    /// ```
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
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn has_weak_key(&self) -> bool
    /// Checks wether or not it has a weak key.
    ///
    /// # Output
    /// This method returns `true` if it has a weak key.
    /// Otherwise, it returns `false`.
    ///
    /// # Example 1 for not weak key
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let weak_key = a_des.has_weak_key();
    /// println!("{:016X} is {}a weak key.", key.to_be(), if weak_key {""} else {"not "});
    /// assert_eq!(weak_key, false);
    /// ```
    ///
    /// # Example 2 for weak key
    /// ```
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_0000000000000000_u64;
    /// a_des.set_key_u64(key);
    /// let weak_key = a_des.has_weak_key();
    /// println!("{:016X} is {}a weak key.", key.to_be(), if weak_key {""} else {"not "});
    /// assert_eq!(weak_key, true);
    /// ```
    #[inline]
    pub fn has_weak_key(&mut self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
    
/*
    // pub fn decrypt_with_padding_pkcs7(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    /// Decrypts the data with the padding defined in PKCS #7 and stores in Vec.
    ///
    /// # Arguments
    /// - `cipher` is the ciphertext to be decrypted, and is a pointer to u8
    ///   which is `*const u8`.
    /// - `length_in_bytes` is the length of the ciphertext `cipher` in bytes,
    ///   and of `u64`-type,
    /// - `message` is the container where the plaintext will be stored, and
    ///   of `*mut u8` type.
    ///
    /// # Output
    /// This method returns the size of the decrypted plaintext in bytes.
    ///
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be less than `8`.
    /// - If `message` has some values before decryption, it will be removed.
    /// - The padding bits is composed of the bytes that indicate the length of
    ///   the plaintext. For more information about the padding bits according
    ///   to PKCS#7, Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - This method performs pure decryption without any operation mode.
    ///   It is equivalent to ECB (Electronic Code Book) mode.
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// ```
    pub fn decrypt_with_padding_pkcs7(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_with_padding_pkcs7_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    /// Decrypts the data with the padding defined in PKCS #7 and stores in Vec.
    ///
    /// # Arguments
    /// - `cipher` is the ciphertext to be decrypted, and is a pointer to u8
    ///   which is `*const u8`.
    /// - `length_in_bytes` is the length of the ciphertext `cipher` in bytes,
    ///   and of `u64`-type,
    /// - `message` is the container where the plaintext will be stored, and
    ///   of `&mut Vec<T>` type.
    ///
    /// # Output
    /// This method returns the size of the decrypted plaintext in bytes.
    ///
    /// # Features
    /// - `length_in_bytes` cannot be less than `8`.
    /// - If `message` has some values before decryption, it will be removed.
    /// - The padding bits is composed of the bytes that indicate the length of
    ///   the plaintext. For more information about the padding bits according
    ///   to PKCS#7, Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - This method performs pure decryption without any operation mode.
    ///   It is equivalent to ECB (Electronic Code Book) mode.
    ///
    /// # Example 1 for Normal case
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba (16 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb (16 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 2 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba (128 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb (128 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 3 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    ///
    /// // Expanded case for 0 rounds which means that key is meaningless
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#018X}", key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    ///
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    ///
    /// let mut recovered1 = Vec::<u8>::new();
    /// let mut recovered2 = Vec::<u8>::new();
    /// c_des.decrypt_with_padding_pkcs7_into_vec(cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    /// d_des.decrypt_with_padding_pkcs7_into_vec(cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    /// print!("B1a (0 rounds) =\t");
    /// for b in recovered1.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    /// print!("B2a (0 rounds) =\t");
    /// for b in recovered2.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    ///
    /// let mut converted1 = String::new();
    /// let mut converted2 = String::new();
    /// unsafe { converted1.as_mut_vec() }.append(&mut recovered1);
    /// unsafe { converted2.as_mut_vec() }.append(&mut recovered2);
    ///
    /// println!("B1b (0 rounds) =\t{}", converted1);
    /// assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted1, message);
    /// println!("B2b (0 rounds) =\t{}", converted2);
    /// assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted2, message);
    /// assert_eq!(converted1, converted1);
    /// ```
    ///
    /// # Example 4 for the message of 0 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 5 for the message shorter than 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for the message of 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for the message of 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    ///
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#018X}", key);
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    ///
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_with_padding_pkcs7_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    /// `message` has to have at least the same size as that of `cipher`.
    ///
    /// # Example 1
    /// ```text
    /// // to do
    /// ```
    pub fn decrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_pkcs7_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn encrypt_with_padding_iso(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_with_padding_iso_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_with_padding_iso_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_iso(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_iso_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_iso_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_iso(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_iso_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_iso_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_iso<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_iso_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_iso_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_iso<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_iso_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_iso_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn decrypt_with_padding_iso(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_iso_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_iso_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_iso_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_iso<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_iso_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_iso_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_iso_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_iso<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_iso_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_iso_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_iso_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn encrypt_with_padding_pkcs7_ecb(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_pkcs7_ecb(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_pkcs7_ecb(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_pkcs7_ecb<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_pkcs7_ecb_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_pkcs7_ecb_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_pkcs7_ecb<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_pkcs7_ecb_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_pkcs7_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn decrypt_with_padding_pkcs7_ecb(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_pkcs7_ecb_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_with_padding_pkcs7_ecb_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_ecb<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_ecb<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_ecb_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_ecb_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

*/
}