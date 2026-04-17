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


use crate::number::SmallUInt;
use crate::asymmetric::PRNG;

/// This trait PKCS1V15 is based on PKCS #1 ver. 1.5. The RSA PKCS #1 v1.5
/// padding format is designed to format a message before RSA encryption to
/// ensure that the resulting block matches the length of the RSA modulus (`n`)
/// and introduces randomness for security.
/// 
/// # PKCS #1 v1.5 Encryption Block Structure
/// The entire Encryption Block (`EB`) is composed of the following fields in
/// order:
/// 
/// | Offset                 | Field Name          | Value          | Description                                                            |
/// |------------------------|---------------------|----------------|------------------------------------------------------------------------|
/// | 0                      | Leading 00          | 0x00           | Ensures the block value is less than the modulus `n`.                  |
/// | 1                      | Block Type (BT)     | 0x02           | Indicates a public-key encryption block (0x01 is used for signatures). |
/// | 2 to `k - m - 4`       | Padding String (PS) | Random bytes   | Non-zero random values (minimum 8 bytes).                              |
/// | `k - m - 2`            | Separator           | 0x00           | Marks the end of padding and the start of the message.                 |
/// | `k - m - 1` to `k - 1` | Data (D)            | Actual Message | The raw data to be encrypted.                                          |
/// 
/// - `k`: The length of the RSA modulus `n` in bytes (e.g., `k = 256`
///   for a 2048-bit key).
/// - `m`: The length of the message `D` in bytes. So, the allowed maximum
///   length of the message in PKCS #1 v1.5 is `k - 11` in bytes (e.g.,
///   `m = 117` for a 1024-bit key, `m = 245` for a 2048-bit key, and
///   `m = 501` for a 4096-bit key).
/// 
/// # Key Field Details
/// ## 1. Leading 0x00 (1 byte)
/// This byte ensures that the integer represented by the block is numerically
/// smaller than the modulus `n`. This is a safety requirement for the RSA
/// operation (M^e (mod n)).
/// 
/// ## 2. Block Type (1 byte)
/// - 0x02: Used for encryption. It requires the Padding String (PS) to be
///   filled with pseudorandom non-zero bytes.
/// - 0x01: Used for digital signatures. The PS is filled with constant `0xFF`
///   bytes.
/// 
/// ## 3. Padding String (Minimum 8 bytes)
/// This string must consist of at least 8 random bytes. Crucially, it must not
/// contain any 0x00 bytes.
/// - Reason: Since 0x00 is used as the Separator, having it inside the padding
///   would confuse the decryption process.
/// - Security: This randomness ensures that encrypting the same message twice
///   results in different ciphertexts (Probabilistic Encryption).
/// 
/// ## 4. Separator (1 byte)
/// A single 0x00 byte. When decrypting, the system scans the block from the
/// beginning and identifies the first 0x00 byte after the Block Type;
/// everything following it is treated as the original message.
/// 
/// # Security Warning: Bleichenbacher's Attack
/// PKCS #1 v1.5 is now considered vulnerable in many modern contexts due to
/// its simple structure.
/// - Padding Oracle Attack: If a server reveals whether a decrypted block has
///   a "correct" or "incorrect" padding format, an attacker can use this
///   information to decrypt ciphertexts without the private key. This is known
///   as Bleichenbacher's attack.
/// - Recommendation: For new systems, it is strongly recommended to use
///   RSA-OAEP, which is mathematically proven to be secure against these types
///   of attacks.
/// 
/// Therefore, PKCS #1 v1.5 is considered not to be cryptographically secure
/// enough so that you are not encouraged to use this trait. Instead, you are
/// encouraged to use the trait OAEP.
pub trait PKCS1V15
{
    const BT: u8 = 2;
    // const BT: u8 = 1;
    // const PS: u8 = 0xFF_u8;

    // fn set_prng(&mut self, prng: impl RNG)
    /// Sets pseudo-random generator engine.
    /// 
    /// # Argument
    /// - **prngn**: an object of pseudo-random generator engine.
    /// 
    /// # Available Pseudo-Random Number Generators (PRNGs)
    /// The available prngs are all `Random_Generic<LESS_SECURE_COUNT>`.
    /// - [`Random_Generic<LESS_SECURE_COUNT>`](../random/struct.Random_Generic.html#struct.Random_Generic)
    /// 
    /// ## You can get a specific PRNG with the following PRNG_Creators:
    /// - [`Asymmetric_PRNG_Creator_BIG_KECCAK_1024`](struct.Asymmetric_PRNG_Creator_BIG_KECCAK_1024.html#struct.Asymmetric_PRNG_Creator_BIG_KECCAK_1024)
    /// - [`Asymmetric_PRNG_Creator_SHA3_512`](struct.Asymmetric_PRNG_Creator_BIG_KECCAK_1024.html#struct.Asymmetric_PRNG_Creator_BIG_KECCAK_1024)
    /// - [`Asymmetric_PRNG_Creator_SHA3_256`](struct.Asymmetric_PRNG_Creator_SHA3_256.html#struct.Asymmetric_PRNG_Creator_SHA3_256)
    /// - [`Asymmetric_PRNG_Creator_SHAKE_256`](struct.Asymmetric_PRNG_Creator_SHAKE_256.html#struct.Asymmetric_PRNG_Creator_SHAKE_256)
    /// - [`Asymmetric_PRNG_Creator_SHAKE_128`](struct.Asymmetric_PRNG_Creator_SHAKE_128.html#struct.Asymmetric_PRNG_Creator_SHAKE_128)
    /// - [`Asymmetric_PRNG_Creator_SHA2_512`](struct.Asymmetric_PRNG_Creator_SHA2_512.html#struct.Asymmetric_PRNG_Creator_SHA2_512)
    /// - [`Asymmetric_PRNG_Creator_SHA2_256`](struct.Asymmetric_PRNG_Creator_SHA2_256.html#struct.Asymmetric_PRNG_Creator_SHA2_256)
    /// - [`Asymmetric_PRNG_Creator_SHA1`](struct.Asymmetric_PRNG_Creator_SHA1.html#struct.Asymmetric_PRNG_Creator_SHA1)
    /// - [`Asymmetric_PRNG_Creator_SHA0`](struct.Asymmetric_PRNG_Creator_SHA0.html#struct.Asymmetric_PRNG_Creator_SHA0)
    /// - [`Asymmetric_PRNG_Creator_MD5`](struct.Asymmetric_PRNG_Creator_MD5.html#struct.Asymmetric_PRNG_Creator_MD5)
    /// - [`Asymmetric_PRNG_Creator_MD4`](struct.Asymmetric_PRNG_Creator_MD4.html#struct.Asymmetric_PRNG_Creator_MD4)
    /// - [`Asymmetric_PRNG_Creator_AES_128`](struct.Asymmetric_PRNG_Creator_AES_128.html#struct.Asymmetric_PRNG_Creator_AES_128)
    /// - [`Asymmetric_PRNG_Creator_DES`](struct.Asymmetric_PRNG_Creator_DES.html#struct.Asymmetric_PRNG_Creator_DES)
    /// - [`Asymmetric_PRNG_Creator_CPRNG_Engine`](struct.Asymmetric_PRNG_Creator_CPRNG_Engine.html#struct.Asymmetric_PRNG_Creator_CPRNG_Engine)
    fn set_prng(&mut self, prng: impl PRNG);

    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext is stored.
    /// - `length_in_bytes` is of `u64`-type, and is the length of the plaintext
    ///   `message` in bytes. Its maximum value is `size_of::<T>() * N - 11`.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `encrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - If `length_in_bytes` is greater than `T::size_in_bytes() * N - 11`,
    ///   this method will not encrypt the message but return `0`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - length_in_bytes - 11` random numbers of
    ///   type `u8`, and x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt)
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to 
    /// PKCS #1 ver. 1.5, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext is stored.
    /// - `length_in_bytes` is of `u64`-type, and is the length of the plaintext
    ///   `message` in bytes. Its maximum value is `size_of::<T>() * N - 11`.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - If `length_in_bytes` is greater than `T::size_in_bytes() * N - 11`,
    ///   this method will not encrypt the message but return `0`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the encrypted data will be stored is enough.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - length_in_bytes - 11` random numbers of
    ///   type `u8`, and x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example 1 for Normal message
    /// click [here](trait@PKCS1V15#method.decrypt_vec)
    fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt;

    // fn encrypt_into_array<U, const M: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; M]) -> u64
    /// Encrypts the data with the padding defined according to 
    /// PKCS #1 ver. 1.5, and stores the encrypted data in array `[U; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext is stored.
    /// - `length_in_bytes` is of `u64`-type, and is the length of the plaintext
    ///   `message` in bytes. Its maximum value is `size_of::<T>() * N - 11`.
    /// - `cipher` is a mutable reference to an array `[U; M]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - If `length_in_bytes` is greater than `T::size_in_bytes() * N - 11`,
    ///   this method will not encrypt the message but return `0`.
    /// - If `U::size_in_bytes()` * `M` is less than `T::size_in_bytes()` * `N`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `M` is equal to `T::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns `T::size_in_bytes()` * `N`.
    /// - If `U::size_in_bytes()` * `M` is greater than
    ///   `T::size_in_bytes()` * `N`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   `T::size_in_bytes()` * `N`.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - length_in_bytes - 11` random numbers of
    ///   type `u8`, and x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example 1 for Normal message
    /// click [here](trait@PKCS1V15#method.decrypt_array)
    fn encrypt_into_array<U, const M: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; M]) -> u64
    where U: SmallUInt;

    // fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object with the padding defined according
    /// to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_into_string)
    #[inline]
    fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in a `str` object with the padding defined according
    /// to PKCS #1 ver. 1.5, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_vec_into_string)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const M: usize>(&mut self, message: &str, cipher: &mut [U; M]) -> u64
    /// Encrypts the data in a `str` object with the padding defined according
    /// to PKCS #1 ver. 1.5, and stores the encrypted data in array `[U; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; M]` object `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `M` is less than `T::size_in_bytes()` * `N`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `M` is equal to `T::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns `T::size_in_bytes()` * `N`.
    /// - If `U::size_in_bytes()` * `M` is greater than
    ///   `T::size_in_bytes()` * `N`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   `T::size_in_bytes()` * `N`.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_array_into_string)
    #[inline]
    fn encrypt_str_into_array<U, const M: usize>(&mut self, message: &str, cipher: &mut [U; M]) -> u64
    where U: SmallUInt
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_string_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("C69D50BE9165760F2218B04D870B1C797E8CF9071548465735E20DD240B50AAF28EFE285E826E989836D3674BDAAD4C18BC0F6C697EA5879CEC372A867073D3B", 16).unwrap();
    /// let prime2 = U512::from_str_radix("921907C5F8773D1922DE00B302C0757DA8136DE1945D02F6386D361CA312FF4A005933C263B96BD68A11DA07D94DD4EDA6412956CF8C51F355945097EF88D4C1", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 128];
    /// rsa.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_into_string(cipher.as_ptr(), &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object with the padding defined
    /// according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("C22E75B2ECE32673042DA138C285720DEE2EAEF521D08E1E3FD3BC50F7D5B8CCB1C2B7FD7A8B9B8A233139B27CC94D7444E84E5F08C92E02DE01FC426E8B0EDF", 16).unwrap();
    /// let prime2 = U512::from_str_radix("EA0752E2BB207AFEF136323991B8BE152FDEB579C07A73999D0D6F20F3836301F2F880007D28356DACCE3E032EB8F64450EB97AAB4D912AFC90919C7E205A45B", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = String::from("In the beginning God created the heavens and the earth.");
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// rsa.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_vec_into_string(&cipher, &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const M: usize>(&mut self, message: &String, cipher: &mut [U; M]) -> u64
    /// Encrypts the data stored in a `String` object with the padding defined
    /// according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; M]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the array `[U; M]`
    ///   object `cipher`.
    /// - If `message.len()` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `M` is less than `T::size_in_bytes()` * `N`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `M` is equal to `T::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns `T::size_in_bytes()` * `N`.
    /// - If `U::size_in_bytes()` * `M` is greater than
    ///   `T::size_in_bytes()` * `N`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   `T::size_in_bytes()` * `N`.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - message.len() - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("EB40B5394E5BDA1BA07FDCAAE0A9B72A855B5C60144A45E0411B43CA7E1DD818E099A9B758DBBF71F5D91C62CD5C9B3A2083BAF67A34F5A3BBB2AADA4FFBCF05", 16).unwrap();
    /// let prime2 = U512::from_str_radix("BB4A66D84474FCC903BF67EF90F455D1C5B1C93B5A9A80D819486C533871940DEB2AB38A9E43733B99219E479CC9C86F8951600ABD95CD305ABEC4325DDC062B", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0u8; 128];
    /// rsa.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_array_into_string(&cipher, &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn encrypt_string_into_array<U, const M: usize>(&mut self, message: &String, cipher: &mut [U; M]) -> u64
    where U: SmallUInt
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `encrypt_vec_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is an empty `Vec<U>` object, `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted.
    /// - If `size_of::<U>() * message.len()` is greater than
    ///   `size_of::<T>() * N - 11`, this method does not perform encryption
    ///   but returns `zero`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - U::size_in_bytes() * message.len() - 11`
    ///   random numbers of type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_into_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt
    {
        self.encrypt(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - If `size_of::<U>() * message.len()` is greater than
    ///   `size_of::<T>() * N - 11`, this method does not perform encryption
    ///   but returns `zero`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - U::size_in_bytes() * message.len() - 11`
    ///   random numbers of type `u8`, and x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1
    ///   ver. 1.5, Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const M: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; M]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<U>() * message.len()` is greater than
    ///   `size_of::<T>() * N - 11`, this method does not perform encryption
    ///   but returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is less than `T::size_in_bytes()` * `N`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to `T::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns `T::size_in_bytes()` * `N`.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `T::size_in_bytes()` * `N`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   `T::size_in_bytes()` * `N`.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - U::size_in_bytes() * message.len() - 11`
    ///   random numbers of type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1
    ///   ver. 1.5, Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_array_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const M: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; M]) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const M: usize>(&mut self, message: &[U; M], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; M]` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; M]` object, and
    ///   is the place where the plaintext is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted.
    /// - If `size_of::<U>() * M` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - size_of::<U>() * M - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_into_array)
    #[inline]
    fn encrypt_array<U, const M: usize>(&mut self, message: &[U; M], cipher: *mut u8) -> u64
    where U: SmallUInt
    {
        self.encrypt(message.as_ptr() as *const u8, (M as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const M: usize>(&mut self, message: &[U; M], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; M]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; M]` object, and
    ///   is the place where the plaintext is stored.
    /// - `cipher` is a mutable reference to `Vec<V>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - If `size_of::<U>() * M` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - size_of::<U>() * M - 11` random numbers of
    ///   type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_vec_into_array)
    #[inline]
    fn encrypt_array_into_vec<U, V, const M: usize>(&mut self, message: &[U; M], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (M as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const L: usize, const M: usize>(&mut self, message: &[U; L], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; L]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; L]` object, and
    ///   is the place where the plaintext is stored.
    /// - `cipher` is a mutable reference to an array `[V; M]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `size_of::<U>() * L` is greater than `size_of::<T>() * N - 11`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is less than `T::size_in_bytes()` * `N`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to `T::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns `T::size_in_bytes()` * `N`.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `T::size_in_bytes()` * `N`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   `T::size_in_bytes()` * `N`.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<T>() * N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes: 0x00_u8, 0x02_u8,
    ///   `T::size_in_bytes() * N - U::size_in_bytes() * L - 11` random
    ///   numbers of type `u8`, and 0x00_u8 according to RFC 2313
    ///   defined in PKCS #1 ver. 1.5.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// click [here](trait@PKCS1V15#method.decrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_array<U, V, const L: usize, const M: usize>(&mut self, message: &[U; L], cipher: &mut [V; M]) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (L as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext is stored.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `decrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<T>() * N - 11`.
    /// - If the size of the area for plaintext is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If the size of the area for plaintext is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If the size of the area for plaintext is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("D70BD5CA4EA8EC514D116988AC3A9C8CF5545B511A2B414F3D6D293B788A6FE264D90D426F681E672235DF0EA0D56BE95C9F992D9ABEF6CD5F370EAF86B0E871", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("D888E920751043B12A95466E0FCB07CE16519BDF0F953547FC37E5E09810E1B7D5D76375A6BD0D9F75C216E616D40ED5C490E7A386FD24594CC1705C14C9E4D7", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key by RSA to get the encrypted certain AES.
    /// // 3. Sends the encrypted certain AES key and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = [0u8; 16];
    ///     for i in 0u8..16
    ///         { key[i as usize] = i; }
    ///     let mut aes = AES_128::new_with_key(&key);
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt(&key as *const u8, 16, &mut encrypted_key as *mut u8);
    ///     (encrypted_key, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key and the ciphertext.
    /// // 2. Decrypts the encrypted AES key by RSA to get the original AES key.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    ///     print!("en_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0; 16];
    ///     rsa.decrypt(encrypted_key.as_ptr(), key.as_mut_ptr());
    ///     print!("key = ");
    ///     for i in 0..16
    ///         { print!("{:X}", key[i]); }
    ///     println!();
    ///     let mut aes = AES_128::new_with_key(&key);
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(&cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64;

    // fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according
    /// to PKCS #1 ver. 1.5, and stores the decrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `decrypt_*_into_vec()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the decrypted data will be stored is enough.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("F00C8AEBB56F5F37FA3A26689DA44E6AF45AC2E6561B0C30E5D6CC0C4ADEFEC86D272A6358D82379EFF88FEED21177E62E06BF68557C2CA0ECD387744A1A8D2D", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("F834AFC1753260793D0AFC137354D4A8696605701D4CA995AF6088AB21036A8789174D3D14A8C64EDBB93ED599F701911B8D245530B06A2A68DF6904ABB1AA01", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key by RSA to get the encrypted certain AES.
    /// // 3. Sends the encrypted certain AES key and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, ECB_ISO };
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = Vec::<u8>::new();
    ///     for i in 0u8..16
    ///         { key.push(i); }
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt_vec(&key, &mut encrypted_key as *mut u8);
    ///     (encrypted_key, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key and the ciphertext.
    /// // 2. Decrypts the encrypted AES key by RSA to get the original AES key.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, ECB_ISO };
    /// 
    ///     print!("Encrypted key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = Vec::<u8>::new();
    ///     rsa.decrypt_into_vec(encrypted_key.as_ptr(), &mut key);
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(&cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
    where U: SmallUInt;

    // fn decrypt_into_array<U, const M: usize>(&mut self, cipher: *const u8, message: &mut [U; M]) -> u64
    /// Decrypts the data with the padding defined according to
    /// PKCS #1 ver. 1.5, and stores the decrypted data in array `[U; M]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[U; M]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - `size_of::<U>() * M` does not have to be more than
    ///   `size_of::<T>() * N - 11`.
    /// - If `size_of::<U>() * M` is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `size_of::<U>() * M` is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If `size_of::<U>() * M` is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("AC3108622D6FB87127289FDF4A996EE10A6F32E4134979275EBB8F358300DE758B91334F6D0FF697773E18927332FFC31537EB746C00EEA0D0BE1C87BF4A78AD", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("FA5FD6A3ACC44C6FA488CBF657F5256582E9C1B539F8BA3730E51DDF0156CDB34C1F9A6DFE2DCD443368EAB8804092BE5C4CBEF3DD28AEEC58D7D8A55C5C2CEB", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };
    ///     use cryptocol::number::SharedArrays;
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = [0u8; 16];
    ///     for i in 0u8..16
    ///         { key[i as usize] = i; }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 1;
    ///     iv[1] = 2;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_add(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(&key);
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt_array(&key, &mut encrypted_key as *mut u8);
    ///     let mut encrypted_iv= [0u8; 128];
    ///     rsa.encrypt_array(&iv, &mut encrypted_iv as *mut u8);
    /// 
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0u8; 16];
    ///     rsa.decrypt_into_array(encrypted_key.as_ptr(), &mut key);
    ///     print!("key = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", key[i]); }
    ///     println!();
    ///     let mut iv = [0u8; 16];
    ///     rsa.decrypt_into_array(encrypted_iv.as_ptr(), &mut iv);
    /// 
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    fn decrypt_into_array<U, const M: usize>(&mut self, cipher: *const u8, message: &mut [U; M]) -> u64
    where U: SmallUInt;

    // fn decrypt_into_string(&mut self, cipher: *const u8, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #1
    /// ver. 1.5, and stores the decrypted data in a `String`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_string().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the decrypted data will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("B7E494B40672CA779511317A9AD1E3707FEC6B62CD4E8E453702FC20B7B09CE9FC6C7B398E42EBC2F9B2F01CC90247259DABDA191E36A7681268503DC70219A9", 16).unwrap();
    /// let prime2 = U512::from_str_radix("EDD1578DEA65D27459F67FF8A7546455C5327DB7A2AEBA51BC4F06843DF93E8CECCF95E92830065FF52833330085285C49B7C2A9AAB3A9F961120612F3D5B0AD", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 128];
    /// rsa.encrypt_str(message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_into_string(cipher.as_ptr(), &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_into_string(&mut self, cipher: *const u8, message: &mut String) -> u64
    {
        self.decrypt_into_vec(cipher, unsafe { message.as_mut_vec() })
    }

    // fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>() * cipher.len()` cannot be other than
    ///   `size_of::<T>() * N`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<T>() * N - 11`.
    /// - If the size of the area for plaintext is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If the size of the area for plaintext is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If the size of the area for plaintext is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("F00C8AEBB56F5F37FA3A26689DA44E6AF45AC2E6561B0C30E5D6CC0C4ADEFEC86D272A6358D82379EFF88FEED21177E62E06BF68557C2CA0ECD387744A1A8D2D", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("F834AFC1753260793D0AFC137354D4A8696605701D4CA995AF6088AB21036A8789174D3D14A8C64EDBB93ED599F701911B8D245530B06A2A68DF6904ABB1AA01", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = Vec::<u8>::new();
    ///     for i in 0u8..16
    ///         { key.push(i); }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = Vec::<u8>::new();
    ///     rsa.encrypt_into_vec(key.as_ptr(), key.len() as u64, &mut encrypted_key);
    ///     let mut encrypted_iv= Vec::<u8>::new();
    ///     rsa.encrypt_into_vec(iv.as_ptr() as *const u8, 16, &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0u8; 16];
    ///     rsa.decrypt_vec(&encrypted_key, key.as_mut_ptr());
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = [0u8; 16];
    ///     rsa.decrypt_vec(&encrypted_iv, iv.as_mut_ptr());
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt
    {
        self.decrypt(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to `Vec<V>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - `size_of::<U>() * cipher.len()` cannot be other than
    ///   `size_of::<T>() * N`.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("EABED4A764F510FC1E5A70DAFCBD67245D61F2B27AA381BA406A8387B805A6AFD144D856A9B7ADE7FDCAED2324C66CB21D565160CEEBF572497C27B6BD1FBCD7", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("8EF1B1D1A88754D1E360D317516E5AE73C617694DB9B36CA6E029B6714537ED09D282AB6601A3A9D6C137503FD7097F7627B20E14CED7CE676734C554F3DB0B9", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = Vec::<u8>::new();
    ///     for i in 0u8..16
    ///         { key.push(i); }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = Vec::<u8>::new();
    ///     rsa.encrypt_vec_into_vec(&key, &mut encrypted_key);
    ///     let mut encrypted_iv= Vec::<u8>::new();
    ///     rsa.encrypt_vec_into_vec(&iv.to_vec(), &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = vec![0u8; 16];
    ///     rsa.decrypt_vec_into_vec(&encrypted_key, &mut key);
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = vec![0u8; 16];
    ///     rsa.decrypt_vec_into_vec(&encrypted_iv, &mut iv);
    ///     let iv: [u8; 16] = iv.try_into().expect("Not fit to the size of array");
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt, V: SmallUInt;

    // fn decrypt_vec_into_array<U, V, const M: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; M]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[V; M]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - `size_of::<U>() * cipher.len()` cannot be other than
    ///   `size_of::<T>() * N`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - If `size_of::<V>() * M` is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `size_of::<V>() * M` is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If `size_of::<V>() * M` is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("A285ADDEA1E045833842C582CF6CFC1780693C6656946E86ABB4D651E9E55D895291CA1B7CFDCFA27BE47095D55EB3FE72A27F40DF5C17E11AA8241092D47F35", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("CC218034E0F69E535885629ED450BFA01DE2407B5EA0ABB51B49587E6A973869D005032C0A5DBA2D174502A0014F8BB767F09AFD4E5495D704E090DA3AED4C89", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = [0u8; 16];
    ///     for i in 0u8..16
    ///         { key[i as usize] = i; }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = Vec::<u8>::new();
    ///     rsa.encrypt_array_into_vec(&key, &mut encrypted_key);
    ///     let mut encrypted_iv= Vec::<u8>::new();
    ///     rsa.encrypt_array_into_vec(&iv, &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0u8; 16];
    ///     rsa.decrypt_vec_into_array(&encrypted_key, &mut key);
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = [0u8; 16];
    ///     rsa.decrypt_vec_into_array(&encrypted_iv, &mut iv);
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(&key);
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_vec_into_array<U, V, const M: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; M]) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_vec_into_string(&mut self, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` with the padding defined according to
    /// PKCS #1 ver. 1.5, and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - `size_of::<U>() * cipher.len()` cannot be other than
    ///   `size_of::<T>() * N`.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// - `cipher.len()` cannot be other than `size_of::<T>() * N`.
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("86CD3DACD10854D23176009D6D3C351D04541E22E0DC48F242FB4D92F08497214002FB4BFEAD9FBD0270C71901B5BBE4C968D0712E741E919134A281B5004AAB", 16).unwrap();
    /// let prime2 = U512::from_str_radix("A2FA368C35E68E3D0A584AD3A671516FE942BC816A6A784346910A48CE81C6C5EF354EC458E91FDEE7D0C1FC3AD66E8011B60CFFFEB1919526EDFC60018E07C5", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// rsa.encrypt_str_into_vec(message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_vec_into_string(&cipher, &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array<U, const M: usize>(&mut self, cipher: &[U; M], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; M]` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; M]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>() * M` cannot be other than `size_of::<T>() * N`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<T>() * N - 11`.
    /// - If the size of the area for plaintext is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If the size of the area for plaintext is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If the size of the area for plaintext is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("9B3D6591A504C40FA6C4BC5465639914FB94A514BA86C399B586BF57822C49A08001DBAACDE3D0D08F5457B12ABB3C1DDCBADF2A91A0F701C14942569AA4BE6B", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("ADC1B51955B59316288B0C23C45DA60D7E808740EF7ABD8D0D21782C7600E0A582D7A81F77959BF52DD4A6BA26ECC1BD8D105273C60DF73738C2D6FCB6E4A99B", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CFB };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = Vec::<u8>::new();
    ///     for i in 0u8..16
    ///         { key.push(i); }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt_into_array(key.as_ptr(), key.len() as u64, &mut encrypted_key);
    ///     let mut encrypted_iv= [0u8; 128];
    ///     rsa.encrypt_into_array(iv.as_ptr() as *const u8, 16, &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, CFB };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0u8; 16];
    ///     rsa.decrypt_array(&encrypted_key, key.as_mut_ptr());
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = [0u8; 16];
    ///     rsa.decrypt_array(&encrypted_iv, iv.as_mut_ptr());
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_array<U, const M: usize>(&mut self, cipher: &[U; M], message: *mut u8) -> u64
    where U: SmallUInt
    {
        self.decrypt(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_vec<U, V, const M: usize>(&mut self, cipher: &[U; M], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; M]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; M]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to `Vec<V>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - `size_of::<U>() * M` cannot be other than `size_of::<T>() * N`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the decrypted data will be stored is enough.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("C17B561347CF845495B16CBDA4994EE2FE843C58AC40B4747411CFEE7A592D8273B5A6A120283D69D8C31FC5088D5B9D7A209B8C4B8048DFBDF10EFE16D3E71D", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("D18734B895C1B6217024B56820FC66D6D7EDC2F5680B1AED9590A856C636BC5F419BD71CFCB91F67CFDFC5ECE1E9494A02FECF054F4F43D8E069C4D5E71B6077", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, OFB };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = [0u8; 16];
    ///     for i in 0u8..16
    ///         { key[i as usize] = i; }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt_array_into_array(&key, &mut encrypted_key);
    ///     let mut encrypted_iv= [0u8; 128];
    ///     rsa.encrypt_array_into_array(&iv, &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, OFB };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = vec![0u8; 16];
    ///     rsa.decrypt_array_into_vec(&encrypted_key, &mut key);
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = vec![0u8; 16];
    ///     rsa.decrypt_array_into_vec(&encrypted_iv, &mut iv);
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let mut iv_ = [0u8; 16];
    ///     for i in 0_usize..16
    ///         { iv_[i] = iv[i]; }
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv_).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_array_into_vec<U, V, const M: usize>(&mut self, cipher: &[U; M], message: &mut Vec<V>) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_array<U, V, const L: usize, const M: usize>(&mut self, cipher: &[U; L], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; L]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; L]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[V; M]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - `size_of::<U>() * L` cannot be other than `size_of::<T>() * N`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - `size_of::<V>() * M` does not have to be more than
    ///   `size_of::<T>() * N - 11`.
    /// - If `size_of::<V>() * M` is less than necessary,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `size_of::<V>() * M` is as big as necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   decrypted data, and returns the size of the plaintext.
    /// - If `size_of::<V>() * M` is greater than necessary,
    ///   this method performs decryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the plaintext.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// // Generates an RSA object
    /// fn generate() -> RSA_1024
    /// {
    ///     let prime1 = U512::from_str_radix("E2080D05BC860FE280F457BB314F9AB284420B6E6B341596E17AB1F69679AFF0D08175DD9392743708F07884BE7CB7962D65BFF3C9FA2B3F13320F48BB1796EF", 16).unwrap();
    ///     let prime2 = U512::from_str_radix("EC8230D29D07370222F3A98487A8F7B5438A07399BE492C2BA2E83E33BB52E7F0FD2404024B130A464FA10DDA5C1FB9C095286E21DBF79AC162C14571DA7DAC5", 16).unwrap();
    ///     RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    /// }
    /// 
    /// // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    /// // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    /// // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    /// fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
    ///     println!("Public Key: {}", rsa.get_public_key());
    ///     println!("Product value: {}", rsa.get_modulus());
    ///     
    ///     let mut key = Vec::<u8>::new();
    ///     for i in 0u8..16
    ///         { key.push(i); }
    ///     let mut iv = [0u8; 16];
    ///     iv[0] = 13;
    ///     iv[1] = 17;
    ///     for i in 2..16
    ///         { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
    ///     let iv_ = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
    ///     let mut cipher = Vec::new();
    ///     aes.encrypt_str_into_vec(iv_.clone(), message, &mut cipher);
    ///     let mut encrypted_key = [0u8; 128];
    ///     rsa.encrypt_vec_into_array(&key, &mut encrypted_key);
    ///     let iv = iv.to_vec();
    ///     let mut encrypted_iv= [0u8; 128];
    ///     rsa.encrypt_vec_into_array(&iv, &mut encrypted_iv);
    ///     (encrypted_key, encrypted_iv, cipher)
    /// }
    /// 
    /// // 1. Receives the encrypted AES key, IV and the ciphertext.
    /// // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    /// // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    /// fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    /// {
    ///     use cryptocol::symmetric::{ AES_128, PCBC_ISO };
    ///     use cryptocol::number::SharedArrays;
    /// 
    ///     print!("Encrypted_key = ");
    ///     for i in 0..128
    ///         { print!("{}", encrypted_key[i]); }
    ///     println!();
    ///     let mut key = [0u8; 16];
    ///     rsa.decrypt_array_into_array(&encrypted_key, &mut key);
    ///     print!("key = ");
    ///     for k in key.clone()
    ///         { print!("{:X}", k); }
    ///     println!();
    ///     let mut iv = [0u8; 16];
    ///     rsa.decrypt_array_into_array(&encrypted_iv, &mut iv);
    ///     print!("iv = ");
    ///     for i in 0_usize..16
    ///         { print!("{:X}", iv[i]); }
    ///     println!();
    ///     let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
    ///     let mut aes = AES_128::new_with_key(&key);
    ///     let mut recovered = String::new();
    ///     aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    ///     println!("Recovered =\t{}", recovered);
    ///     recovered
    /// }
    /// 
    /// let rsa = generate();
    /// let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    /// let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_array_into_array<U, V, const L: usize, const M: usize>(&mut self, cipher: &[U; L], message: &mut [V; M]) -> u64
    where U: SmallUInt, V: SmallUInt
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_string<U, const M: usize>(&mut self, cipher: &[U; M], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; M]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; M]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - `size_of::<U>() * M` cannot be other than `size_of::<T>() * N`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the decrypted data will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// - For more information about the padding bits according to PKCS #1 ver. 1.5,
    ///   Read [here](https://datatracker.ietf.org/doc/html/rfc2313).
    /// 
    /// # Example for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let prime1 = U512::from_str_radix("DA97ECE2D8B5B50AB6263044B74054EE6E14DE616A6102DD53C6047DD08C38C234289851DE79B50F963CDC5D7A0C1DFA20CD9CDE3D61C4871A1F55E40CBB0E9F", 16).unwrap();
    /// let prime2 = U512::from_str_radix("A5F59876FE0FE52E2017BF43E181739DB88C76DDA8F834D25B6B9A3213464A0AC46B742D971677F187C7ED45091127E62FA13ADFBC2A96F3368652EC9D963D4B", 16).unwrap();
    /// let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("Private Key: {}", rsa.get_private_key());
    /// println!("Public Key: {}", rsa.get_public_key());
    /// println!("Product value: {}", rsa.get_modulus());
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0u8; 128];
    /// rsa.encrypt_str_into_array(message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = String::new();
    /// rsa.decrypt_array_into_string(&cipher, &mut recovered);
    /// println!("Recovered =\t {}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    #[inline]
    fn decrypt_array_into_string<U, const M: usize>(&mut self, cipher: &[U; M], message: &mut String) -> u64
    where U: SmallUInt
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }
}
