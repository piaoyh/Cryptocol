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
#![allow(non_camel_case_types)]
// #![warn(rustdoc::missing_doc_code_examples)]

use crate::number::{ TraitsBigUInt, BigUInt, BigUInt_Modular, BigUInt_Prime };
use crate::random::Random;


pub type RSA_4096_u128 = RSA_Generic<32, u128>;
pub type RSA_2048_u128 = RSA_Generic<16, u128>;
pub type RSA_1024_u128 = RSA_Generic<8, u128>;

pub type RSA_4096_u64 = RSA_Generic<64, u64>;
pub type RSA_2048_u64 = RSA_Generic<32, u64>;
pub type RSA_1024_u64 = RSA_Generic<16, u64>;

pub type RSA_4096_u32 = RSA_Generic<128, u32>;
pub type RSA_2048_u32 = RSA_Generic<64, u32>;
pub type RSA_1024_u32 = RSA_Generic<32, u32>;

pub type RSA_4096_u16 = RSA_Generic<256, u16>;
pub type RSA_2048_u16 = RSA_Generic<128, u16>;
pub type RSA_1024_u16 = RSA_Generic<64, u16>;

pub type RSA_4096_u8 = RSA_Generic<512, u8>;
pub type RSA_2048_u8 = RSA_Generic<256, u8>;
pub type RSA_1024_u8 = RSA_Generic<128, u8>;

pub type RSA_4096 = RSA_4096_u32;
pub type RSA_2048 = RSA_2048_u32;
pub type RSA_1024 = RSA_1024_u32;

/// RSA (Ron Rivest, Adi Shamir, Leonard Adleman) is asymmetric-key
/// encryption/decryption algorithm for the encryption of digital data
/// 
/// # Note
/// __This descryption about RSA is according to little endianness.__
/// MSB (Most Significant Bit) is the highest-order bit and LSB (Least
/// Significant Bit) is the lowest-order bit in this descryption unless
/// otherwise mentioned.
/// 
/// # Introduction
/// RSA is an asymmetric-key encryption/decryption algorithm which was invented
/// by three brilliant cryptographers: Ron Rivest, Adi Shamir, and Leonard
/// Adleman.
/// 
/// Unlike symmetric key encryption/decryption algorithm, asymmetric-key
/// encryption/decryption algorithm has two keys: encryption key and decryption
/// key. RSA is one of the asymmetric-key encryption/decryption algorithms.
/// 
/// The encryption key is also known as public key because the encryption key
/// is normally publicized and the decryption key shpuld be kept secrete.
/// Nobody except its owner should have the private key or the decryption key.
/// 
/// So, everybody can encrypt a message with a public key which is publicized
/// for encrytion but only the owner of a private key can decrypt
/// the ciphertext which the message was encrypted into with the public key. 
/// 
/// # Why RSA?
/// The symmetric key encryption/decryption algorithm has two critical issues:
/// key distribution and key management.
/// 
/// Suppose that you use AES to communicate with your friends for example. How
/// will you share the key with your friends? You will have to distribute the
/// key physically. You can't send it to your friends online. If you send it to
/// your friends online, you have to assume that your enemy will eavesdrop your
/// communication and get the key too.
/// 
/// If `N` people communicate with one another in security, each one has to
/// manage `N - 1` keys and they should not lose them. Once even one of them is
/// lost, two people of them should meet again to share a new key.
/// 
/// RSA (actually not only RSA but also all asymmetric key encryption/decryption
/// algorithms) will be the good solution for both the key distribution problem
/// and the key management problem! How? All you have to do is encrypt the
/// symmetric key with the public key of your recipient and send it to your
/// recipient. Then, your recipient will decrypt it and get the symmetric key.
/// Now, you will succeed in sharing the symmetric key safely.
/// 
/// Whenever you communicate with your friend, you can create a new symmetic key
/// and send it to your friend with RSA encryption. Then, you don't have to
/// manage all the symmetric keys without any fear of losing the symmetric keys.
/// 
/// # Short History of birth of RSA
/// // todo
/// 
/// # Use of RSA and its variants
/// // todo
/// 
/// # Generic Parameters
/// RSA_Generic uses BigUInt<T, N> inside. The generic parameters `N` and `T`
/// are same generic parameter of BigUInt<T, N>.
/// - `T`: is supposed to be a _primitive unsigned integer_ type such as `u8`,
///   `u16`, `u32`, `u64`, `u128` and `usize`. So, you are supposed to choose
///   one of `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
/// - `N`: is related with the length of the BigUInt. The total length of
///   BigUInt is `size_of::<T>() * N`.
/// - `MR`: is the repetition of Millar-Rabin algorithm to find a prime number.
/// 
/// # Reference
/// [Read more](https://en.wikipedia.org/wiki/RSA_cryptosystem)
/// about RSA in brief.
/// Watch [this video](https://www.youtube.com/watch?v=fq6SXByItUI) and then
/// [this video](https://www.youtube.com/watch?v=QSlWzKNbKrU) in series
/// for more (or deeper or full) understanding of RSA.
/// 
/// # Quick Start
/// You have to import (use) one of the modules: `RSA_4096`, `RSA_2048`, and
/// `RSA_1024` in order to use official RSA as shown in Example 1. You can also
/// use your own variation of RSA. Then, you have to import (use) the module
/// `RSA_Generic` as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::asymmetric::RSA_1024;
/// use cryptocol::asymmetric::RSA_2048;
/// use cryptocol::asymmetric::RSA_4096;
/// use cryptocol::asymmetric::RSA_Genric;
/// use cryptocol::asymmetric::RSA_1024_u128;
/// use cryptocol::asymmetric::RSA_2048_u128;
/// use cryptocol::asymmetric::RSA_4096_u128;
/// use cryptocol::asymmetric::RSA_1024_u64;
/// use cryptocol::asymmetric::RSA_2048_u64;
/// use cryptocol::asymmetric::RSA_4096_u64;
/// use cryptocol::asymmetric::RSA_1024_u32;
/// use cryptocol::asymmetric::RSA_2048_u32;
/// use cryptocol::asymmetric::RSA_4096_u32;
/// use cryptocol::asymmetric::RSA_1024_u16;
/// use cryptocol::asymmetric::RSA_2048_u16;
/// use cryptocol::asymmetric::RSA_4096_u16;
/// use cryptocol::asymmetric::RSA_1024_u8;
/// use cryptocol::asymmetric::RSA_2048_u8;
/// use cryptocol::asymmetric::RSA_4096_u8;
/// ```
/// 
/// # Notice for Practical Use
/// 
pub struct RSA_Generic<const N: usize, T, const MR: usize = 5>
where T: TraitsBigUInt<T>
{
    modulo: BigUInt<T, N>,
    key_public: BigUInt<T, N>,
    key_private: BigUInt<T, N>,
}

impl<const N: usize, T, const MR: usize> RSA_Generic<N, T, MR>
where T: TraitsBigUInt<T>
{
    // pub fn new() -> Self
    /// Constructs a new object of the struct `RSA_Generic`.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic` whose public key and
    /// private key are all reset to be `zero`.
    /// 
    /// # Features
    /// Unless you set the public key and the private key to be proper values,
    /// you cannot use the object generated by this method.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// let rsa = RSA_1024::new();
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new)
    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
        }
    }

    // pub fn new_with_automatic_keys() -> Self
    /// Constructs a new object of the struct `RSA_Generic` with proper keys.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic`
    /// whose public key and private key are all automatically chosen.
    /// 
    /// # Features
    /// The public key and the private key are automatically set to be proper
    /// values,
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// 
    /// let rsa = RSA_1024::new_with_automatic_keys();
    /// let private_key = rsa.get_private_key();
    /// let public_key = rsa.get_public_key();
    /// let modulo = rsa.get_modulo();
    /// println!("private key = {:X}:{:x}", private_key, modulo);
    /// println!("public key = {:X}:{:x}", public_key, modulo);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new_with_automatic_keys)
    pub fn new_with_automatic_keys() -> Self
    {
        let mut rsa = Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
        };
        rsa.find_keys();
        rsa
    }

    // pub fn new_with_keys(key_public: BigUInt<T, N>, key_private: BigUInt<T, N>, modulo: BigUInt<T, N>) -> Self
    /// Constructs a new object of the struct `RSA_Generic` with given keys.
    /// 
    /// # Arguments
    /// - `key_public` is the exponenent part of the public key,
    ///   and is of type `BigUInt<T, N>`.
    /// - `key_private` is the exponenent part of the private key,
    ///   and is of type `BigUInt<T, N>`.
    /// - `modulo` is the product part the two prime numbers of the private key
    ///   and the public key, and is of type `BigUInt<T, N>`.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic`
    /// whose public key and private key are all given.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let private_key = U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap();
    /// let public_key = U1024::from(7_u8);
    /// let modulo = U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap();
    ///
    /// let rsa = RSA_1024::new_with_keys(public_key, private_key, modulo);
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new_with_keys)
    #[inline]
    pub fn new_with_keys(key_public: BigUInt<T, N>, key_private: BigUInt<T, N>, modulo: BigUInt<T, N>) -> Self
    {
        Self
        {
            modulo,
            key_public,
            key_private,
        }
    }

    // pub fn new_with_primes<const M: usize>(prime_1: BigUInt<T, M>, prime_2: BigUInt<T, M>) -> Self
    /// Constructs a new object of the struct `RSA_Generic`
    /// from given prime numbers.
    /// 
    /// # Arguments
    /// - `prime_1` is one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// - `prime_2` is the other one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic`
    /// whose public key and private key are all given.
    /// 
    /// # Caution
    /// M should be N/2.
    /// Otherwise, the performance of this module may not guaranteed.
    /// 
    /// # Example 1 for RSA_1024_u16
    /// ```
    /// use cryptocol::asymmetric::RSA_1024_u16;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
    /// let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();
    /// 
    /// let rsa = RSA_1024_u16::new_with_primes(prime1, prime2);
    /// println!("RSA_1024_u16: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024_u16: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new_with_primes)
    pub fn new_with_primes<const M: usize>(prime_1: BigUInt<T, M>, prime_2: BigUInt<T, M>) -> Self
    {
        let mut rsa = Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
        };
        rsa.calculate_keys(prime_1, prime_2);
        rsa
    }

    // pub fn new_with_prepared_keys() -> Self
    /// Constructs a new object of the struct `RSA_Generic`
    /// from prepared prime numbers.
    /// 
    /// # Arguments
    /// - `prime_1` is one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// - `prime_2` is the other one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic`
    /// whose public key and private key are all given.
    /// 
    /// # Caution and Cryptographical Security
    /// The source code of this method is openly publicised. It means that
    /// anyone can have the full familiarity of the prime number pool which
    /// this method uses. So, you are NOT encouraged to use this method for
    /// serious cryptographical purposes. You can use this method temporarily
    /// for testing or debugging purposes because of the slowness of the
    /// Millar-Rabin algorithm for big numbers.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// 
    /// let rsa = RSA_1024::new_with_prepared_keys();
    /// let private_key = rsa.get_private_key();
    /// let public_key = rsa.get_public_key();
    /// let modulo = rsa.get_modulo();
    /// println!("RSA_1024: private key = {:X}:{:x}", private_key, modulo);
    /// println!("RSA_1024: public key = {:X}:{:x}", public_key, modulo);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new_with_prepared_keys)
    pub fn new_with_prepared_keys() -> Self
    {
        let length = T::size_in_bytes() as usize * N;
        let mut rand = Random::new();
        let prime_1: BigUInt<T, N> = rand.prepared_random_prime_with_half_length();
        let mut prime_2 = rand.prepared_random_prime_with_half_length();
        while prime_1 == prime_2
            { prime_2 = rand.prepared_random_prime_with_half_length(); }
        Self::new_with_primes(prime_1, prime_2)
    }

    #[inline]
    pub fn get_public_key(&self) -> BigUInt<T, N>
    {
        self.key_public.clone()
    }

    #[inline]
    pub fn get_private_key(&self) -> BigUInt<T, N>
    {
        self.key_private.clone()
    }

    #[inline]
    pub fn get_modulo(&self) -> BigUInt<T, N>
    {
        self.modulo.clone()
    }

    #[inline]
    pub fn set_public_key(&mut self, key_public: BigUInt<T, N>)
    {
        self.key_public = key_public;
    }

    #[inline]
    pub fn set_private_key(&mut self, key_private: BigUInt<T, N>)
    {
        self.key_private = key_private;
    }

    #[inline]
    pub fn set_modulo(&mut self, modulo: BigUInt<T, N>)
    {
        self.modulo = modulo;
    }

    /// # Caution
    /// M should be N/2. Otherwise, the performance of this module may not
    /// guaranteed .
    fn choose_prime_numbers() -> (BigUInt<T, N>, BigUInt<T, N>)
    {
        let mut rand = Random::new();
        let prime_1: BigUInt<T, N> = rand.random_prime_with_half_length_using_miller_rabin_biguint(MR);
        let mut prime_2 = rand.random_prime_with_half_length_using_miller_rabin_biguint(MR);
        while prime_1 == prime_2
            { prime_2 = rand.random_prime_with_half_length_using_miller_rabin_biguint(MR); }
        (prime_1, prime_2)
    }

    pub fn find_keys(&mut self)
    {
        let (prime_1, prime_2) = Self::choose_prime_numbers();
        self.calculate_keys(prime_1, prime_2);
    }

    /// # Caution
    /// M should be N/2. Otherwise, the performance of this module may not
    /// guaranteed .
    pub fn calculate_keys<const M: usize>(&mut self, prime_1: BigUInt<T, M>, prime_2: BigUInt<T, M>)
    {
        self.modulo = prime_1.expanding_mul(&prime_2);
        let phi = prime_1.wrapping_sub_uint(1_u8).expanding_mul(&prime_2.wrapping_sub_uint(1_u8));
        self.key_public = BigUInt::<T, N>::from_uint(2_u8);
        let mut one: BigUInt<T, N>;
        (one, self.key_private, _) = self.key_public.extended_gcd(&phi);

        while !one.is_one()
        {
            self.key_public.wrapping_add_assign_uint(1_u8);
            (one, self.key_private, _) = self.key_public.extended_gcd(&phi);
        }
        if self.key_private.is_underflow()
            { self.key_private = phi.wrapping_sub(&BigUInt::<T, N>::zero().wrapping_sub(&self.key_private)); }
        else
            { self.key_private.wrapping_rem_assign(&phi); }
    }

    // pub fn encrypt_biguint(&self, message: &BigUInt<[T, N>) -> BigUInt<T, N>
    /// Encrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is of the type `&BigUInt<T, N>`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the form of `BigUInt<T, N>`.
    #[inline]
    pub fn encrypt_biguint(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        message.modular_pow(&self.key_public, &self.modulo)
    }

    // pub fn decrypt_biguint(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Decrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&BigUInt<T, N>`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the form of `BigUInt<T, N>`.
    #[inline]
    pub fn decrypt_biguint(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        cipher.modular_pow(&self.key_private, &self.modulo)
    }

    // pub fn encrypt_array_biguint<const M: usize>(&self, message: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    /// Encrypts data in the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is of the type `&[BigUInt<T, N>; M]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data as the array of `BigUInt<T, N>`.
    ///
    /// # Caution
    /// This method is very impractical. Normally, RSA is extremely slow
    /// in encryption/decryption compared to AES. So, almost nobody would use
    /// RSA in the same way of AES. You are not encourged to use this
    /// method unless you really have to use this method.
    pub fn encrypt_array_biguint<const M: usize>(&self, message: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    {
        let mut cipher = message.clone();
        for i in 0..M
            { cipher[i] = self.encrypt_biguint(&message[i]); }
        cipher
    }

    // pub fn decrypt_array_biguint(&self, cipher: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    /// Decrypts data in the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[BigUInt<T, N>; M]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data as the array of `BigUInt<T, N>`.
    ///
    /// # Caution
    /// This method is very impractical. Normally, RSA is extremely slow
    /// in encryption/decryption compared to AES. So, almost nobody would use
    /// RSA in the same way of AES. You are not encourged to use this
    /// method unless you really have to use this method.
    pub fn decrypt_array_biguint<const M: usize>(&self, cipher: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    {
        let mut message = cipher.clone();
        for i in 0..M
            { message[i] = self.decrypt_biguint(&cipher[i]); }
        message
    }

    // pub fn encrypt_unit(&self, message: &[T; N]) -> [T; N]
    /// Encrypts data in the array of `T`.
    ///
    /// # Arguments
    /// `message` is of the type `&[T; N]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data as the array of `T`.
    pub fn encrypt_unit(&self, message: &[T; N]) -> [T; N]
    {
        let mut m = BigUInt::<T, N>::from_array(message.clone());
        m.to_be_assign();
        let mut c = self.encrypt_biguint(&m);
        c.to_be_assign();
        *c.get_number()
    }

    // pub fn decrypt_unit(&self, cipher: &[T; N]) -> [T; N]
    /// Decrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&BigUInt<T, N>`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `BigUInt<T, N>`.
    #[inline]
    pub fn decrypt_unit(&self, cipher: &[T; N]) -> [T; N]
    {
        let mut c = BigUInt::<T, N>::from_array(cipher.clone());
        c.to_be_assign();
        let mut m = self.decrypt_biguint(&c);
        m.to_be_assign();
        *m.get_number()
    }

    // pub fn encrypt_array_unit<const M: usize>(&self, message: &[[T; N]; M]) -> [[T; N]; M]
    /// Encrypts data in the array of `[T; N]`.
    ///
    /// # Arguments
    /// `message` is of the type `&[[T; N]; M]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the array of `[T; N]`.
    ///
    /// # Caution
    /// This method is very impractical. Normally, RSA is extremely slow
    /// in encryption/decryption compared to AES. So, almost nobody would use
    /// RSA in the same way of AES. You are not encourged to use this
    /// method unless you really have to use this method.
    pub fn encrypt_array_unit<const M: usize>(&self, message: &[[T; N]; M]) -> [[T; N]; M]
    {
        let mut cipher = [[T::zero(); N]; M];
        for i in 0..M
            { cipher[i] = self.encrypt_unit(&message[i]); }
        cipher
    }

    // pub fn decrypt_array_unit(&self, cipher: &[[T; N]; M]) -> [[T; N]; M]
    /// Decrypts data in the array of `[T; N]`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[[T; N]; M]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `[T; N]`.
    ///
    /// # Caution
    /// This method is very impractical. Normally, RSA is extremely slow
    /// in encryption/decryption compared to AES. So, almost nobody would use
    /// RSA in the same way of AES. You are not encourged to use this
    /// method unless you really have to use this method.
    #[inline]
    pub fn decrypt_array_unit<const M: usize>(&self, cipher: &[[T; N]; M]) -> [[T; N]; M]
    {
        let mut message = [[T::zero(); N]; M];
        for i in 0..M
            { message[i] = self.decrypt_unit(&cipher[i]); }
        message
    }

    // // pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    // ///
    // #[inline]
    // pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     message.to_be().modular_pow(&self.key_private, &modulo).to_be()
    // }

    // // pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // ///
    // #[inline]
    // pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     cipher.to_be().modular_pow(&self.key_public, &modulo).to_be()
    // }

    // pub fn verify_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     cipher.to_be().modular_pow(&self.key_public, &modulo).to_be()
    // }
}
