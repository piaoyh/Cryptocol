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


use crate::number::SmallUInt;


/// This trait PKCS1V15 is based on PKCS #1 ver. 1.5. It is considered not
/// to be cryptographically secure enough. So, you are not encouraged to use
/// this trait. Intead, you are encouraged to use the trait OAEP.
pub trait PKCS1V15
{
    const BT: u8 = 2;
    // const BT: u8 = 1;
    // const PS: u8 = 0xFF_u8;

    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `encrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt) */
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to 
    /// PKCS #1 ver. 1.5, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is a null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_into_vec) */
    fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to 
    /// PKCS #1 ver. 1.5, and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null data.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_into_array) */
    fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

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
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_str) */
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
/*  /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_str_into_vec) */
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object with the padding defined according
    /// to PKCS #1 ver. 1.5, and stores the encrypted data in array `[U; N]`.
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
/*  /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_str_into_array) */
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
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
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_string_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_string) */
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
/*  /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_string_into_vec) */
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object with the padding defined
    /// according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<U>()` * `N` is less than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<U>()` * `N` is equal to `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<U>()` * `N` is greater than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_string_into_array) */
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
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
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_vec) */
    #[inline]
    fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
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
/*  /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_vec_into_vec) */
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<V>()` * `N` is greater than
    ///   `size_of::<U>() * message.len()`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_vec_into_array) */
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
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
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `N` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_array) */
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
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
/*  /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_array_into_vec) */
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - If this method succeeds in encryption, the output will be
    ///   `size_of::<T>() * N`.
    /// - If this method failed in encryption, this method returns `zero`.
/*  /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes()` * `M` is less than 
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of the
    ///   array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.encrypt_array_into_array) */
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
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
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `decrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `length_in_bytes` - `1`, the rest of the area will be filled with
    ///   `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt) */
    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64;

    // fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according
    /// to PKCS #1 ver. 1.5, and stores the decrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
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
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_into_vec) */
    fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined
    /// according to PKCS #1 ver. 1.5, and stores the decrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes` - `1`,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to or greater than
    ///   `length_in_bytes` - `1`, this method performs decryption, fills the
    ///   array `message` with the decrypted data, and then fills the rest of
    ///   the elements of the array `message` with zeros, and returns the size
    ///   of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_into_array) */
    fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #1
    /// ver. 1.5, and stores the decrypted data in a `String`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
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
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_string().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_into_string) */
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `cipher.len()` cannot be other than any multiple
    ///   of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `cipher.len()` - `1`, the rest of the area will be
    ///   filled with `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_vec) */
    #[inline]
    fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_vec_into_vec) */
    fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone;

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #1 ver. 1.5, and
    /// stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>()` * `cipher.len()` - `1`, this method does not perform
    ///   decryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to or greater than
    ///   `size_of::<U>()` * `cipher.len()` - `1`, this method performs
    ///   decryption, fills the array `message` with the decrypted data, and then
    ///   fills the rest of the elements of the array `message` with zeros, and
    ///   returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_vec_into_array) */
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_vec_into_string) */
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding defined
    /// according to PKCS #1 ver. 1.5.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
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
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `N` cannot be other than any multiple
    ///   of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `N` - `1`, the rest of the area will be
    ///   filled with `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_array) */
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
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
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_array_into_vec) */
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - If `size_of::<V>()` * `M` is less than 
    ///   `size_of::<U>()` * `N` - `1`, this method does not perform
    ///   decryption but returns `zero`.
    /// - If `size_of::<V>()` * `M` is equal to or greater than
    ///   `size_of::<U>()` * `N` - `1`, this method performs decryption,
    ///   fills the array `message` with the decrypted data, and then
    ///   fills the rest of the elements of the array `message` with zeros, and
    ///   returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_array_into_array) */
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, message)
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #1 ver. 1.5,
    /// and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
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
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
/*  /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/pkcs1v15/struct.RSA_Generic.html#method.decrypt_array_into_string) */
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }
}
