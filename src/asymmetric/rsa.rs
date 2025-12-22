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
///    Its default value is `7` for 99.99% accuracy.
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
pub struct RSA_Generic<const N: usize, T, const MR: usize = 7>
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
    /// - `modulo` is the common part of the private key and the public key, and
    ///    is of type `BigUInt<T, N>`, which is the product of two prime numbers
    ///    for generating the private key and the public key.
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

    // pub fn new_with_primes(prime_1: BigUInt<T, N>, prime_2: BigUInt<T, N>) -> Self
    /// Constructs a new object of the struct `RSA_Generic`
    /// from given prime numbers.
    /// 
    /// # Arguments
    /// - `prime_1` is one of the two prime numbers, and is of type
    ///   `BigUInt<T, N>` with the length `T::size_in_bytes() * N / 2`.
    /// - `prime_2` is the other one of the two prime numbers, and is of type
    ///   `BigUInt<T, N>` with the length `T::size_in_bytes() * N / 2`.
    /// 
    /// # Output
    /// A new object of the struct `RSA_Generic`
    /// whose public key and private key are all given.
    /// 
    /// # Caution
    /// The lengths of `prime_1` and `prime_2` should be
    /// `T::size_in_bytes() * N / 2`.
    /// Otherwise, the performance of this module may not guaranteed.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let prime1 = U512::from_str_radix("FF119CE47EE8EBDC06F87902569A80FBB66DF6514FB6CFAE8210ADBD4F9D4071", 16).unwrap();
    /// let prime2 = U512::from_str_radix("975BA2539AA5CE0A8AFB43EDDDBDE7EE6432274E8DC17F6A3543DFBBAA3ED30B", 16).unwrap();
    /// 
    /// let rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.new_with_primes)
    pub fn new_with_primes(prime_1: BigUInt<T, N>, prime_2: BigUInt<T, N>) -> Self
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

    // pub fn get_public_key(&self) -> BigUInt<T, N>
    /// Returns the exponenent part of the public key
    /// 
    /// # Output
    /// The exponenent part of the public key of the type `BigUInt<T, N>`
    /// 
    /// # Example 1
    /// click [here](struct@RSA_Generic#method.new_with_prepared_keys)
    #[inline]
    pub fn get_public_key(&self) -> BigUInt<T, N>
    {
        self.key_public.clone()
    }

    // pub fn get_private_key(&self) -> BigUInt<T, N>
    /// Returns the exponenent part of the private key
    /// 
    /// # Output
    /// The exponenent part of the private key of the type `BigUInt<T, N>`
    /// 
    /// # Example 1
    /// click [here](struct@RSA_Generic#method.new_with_prepared_keys)
    #[inline]
    pub fn get_private_key(&self) -> BigUInt<T, N>
    {
        self.key_private.clone()
    }

    // pub fn get_modulo(&self) -> BigUInt<T, N>
    /// Returns the common part of the private key and the public key
    /// 
    /// # Output
    /// The common part of the private key and the public key,
    /// which is the product of two prime numbers for generating the private key
    /// and the public key, and is of type `BigUInt<T, N>`.
    /// 
    /// # Example 1
    /// click [here](struct@RSA_Generic#method.new_with_prepared_keys)
    #[inline]
    pub fn get_modulo(&self) -> BigUInt<T, N>
    {
        self.modulo.clone()
    }

    // pub fn set_public_key(&mut self, key_public: BigUInt<T, N>)
    /// Sets the exponenent part of the public key
    /// 
    /// # Arguments
    /// `key_public` is the exponenent part of the public key,
    /// and is of type `BigUInt<T, N>`.
    /// 
    /// # Caution
    /// - `key_public` cannot be an arbitrary `BigUInt<T, N>`-typed number
    ///   but is supposed to be well-designed `BigUInt<T, N>`-typed number
    ///   derived from two big prime numbers.
    /// - If `key_public` is an arbitrary `BigUInt<T, N>`-typed number,
    ///   the behaviour of the most methods of `Self` are not defined.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rsa = RSA_1024::new();
    /// rsa.set_public_key(U1024::from(7_u8));
    /// rsa.set_private_key(U1024::from_str_radix("6F21015F58239A612E66501D445CC4F221AB05A16AD9BCD5F3494E44397BCDED845BA62788A4B6C4E008A120F0FD7D45E96CFDB6CC0EF621889A39BA55037EE32946DCB260FB638EF573CA2F4BB3C922A91EA2719E0BC1268410E862D1A2BDF7317970DBDA5AC089FFA1A74DC097AAF08AEEEFAF34DDB1DF172B0743D43B0B", 16).unwrap());
    /// rsa.set_modulo(U1024::from_str_radix("c772e0c82db6549484796c056c18b9ee836f8504611a3792df4b71e78a65992e6366f1f3024dd571b591cdd2a7d7e19a52d1ff6ffff40ba2fda3229ead5e90f2af877de674a7443acfbbeb3a3e5b1968cef3f6bdf0639edaa94174fa8c6d4701fca2d46a041f6ea3e0e921c70434781fd49f1b31f6cc2970aeefc981490b47e5", 16).unwrap());
    /// let private_key = rsa.get_private_key();
    /// let public_key = rsa.get_public_key();
    /// let modulo = rsa.get_modulo();
    /// println!("RSA_1024: private key = {:X}:{:x}", private_key, modulo);
    /// println!("RSA_1024: public key = {:X}:{:x}", public_key, modulo);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.set_public_key)
    #[inline]
    pub fn set_public_key(&mut self, key_public: BigUInt<T, N>)
    {
        self.key_public = key_public;
    }

    // pub fn set_private_key(&mut self, key_private: BigUInt<T, N>)
    /// Sets the exponenent part of the private key
    /// 
    /// # Arguments
    /// `key_private` is the exponenent part of the private key,
    /// and is of type `BigUInt<T, N>`.
    /// 
    /// # Caution
    /// - `key_private` cannot be an arbitrary `BigUInt<T, N>`-typed number
    ///   but is supposed to be well-designed `BigUInt<T, N>`-typed number
    ///   derived from two big prime numbers.
    /// - If `key_private` is an arbitrary `BigUInt<T, N>`-typed number,
    ///   the behaviour of the most methods of `Self` are not defined.
    /// 
    /// # Example 1 for RSA_1024
    /// click [here](struct@RSA_Generic#method.set_public_key)
    #[inline]
    pub fn set_private_key(&mut self, key_private: BigUInt<T, N>)
    {
        self.key_private = key_private;
    }

    // pub fn set_modulo(&mut self, modulo: BigUInt<T, N>)
    /// Sets the common part of the private key and the public key
    /// 
    /// # Arguments
    /// `modulo` is the common part of the private key and the public key, and
    /// is of type `BigUInt<T, N>`, which is the product of two prime numbers
    /// for generating the private key and the public key.
    /// 
    /// # Caution
    /// - `modulo` cannot be an arbitrary `BigUInt<T, N>`-typed number
    ///   but is supposed to be well-designed `BigUInt<T, N>`-typed number
    ///   derived from two big prime numbers.
    /// - If `modulo` is an arbitrary `BigUInt<T, N>`-typed number,
    ///   the behaviour of the most methods of `Self` are not defined.
    /// 
    /// # Example 1 for RSA_1024
    /// click [here](struct@RSA_Generic#method.set_public_key)
    #[inline]
    pub fn set_modulo(&mut self, modulo: BigUInt<T, N>)
    {
        self.modulo = modulo;
    }

    #[inline]
    fn choose_prime_numbers() -> (BigUInt<T, N>, BigUInt<T, N>)
    {
        crate::random::Random::new().random_prime_with_half_length_using_rsa_biguint(MR)
    }

    // pub fn find_keys(&mut self)
    /// Finds (decides) proper keys automatically.
    /// 
    /// # Features
    /// The public key and the private key are automatically set to be proper
    /// values.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// 
    /// let mut rsa = RSA_1024::new();
    /// rsa.find_keys();
    /// let private_key = rsa.get_private_key();
    /// let public_key = rsa.get_public_key();
    /// let modulo = rsa.get_modulo();
    /// println!("RSA_1024: private key = {:X}:{:x}", private_key, modulo);
    /// println!("RSA_1024: public key = {:X}:{:x}", public_key, modulo);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.find_keys)
    pub fn find_keys(&mut self)
    {
        let (prime_1, prime_2) = Self::choose_prime_numbers();
        self.calculate_keys(prime_1, prime_2);
    }

    // pub fn calculate_keys(&mut self, prime_1: BigUInt<T, N>, prime_2: BigUInt<T, N>)
    /// Calculates keys from the given prime numbers
    /// 
    /// # Arguments
    /// - `prime_1` is one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// - `prime_2` is the other one of the two prime numbers,
    ///   and is of type `BigUInt<T, M>` where M is N/2.
    /// 
    /// # Caution
    /// The lengths of `prime_1` and `prime_2` should be
    /// `T::size_in_bytes() * N / 2`.
    /// Otherwise, the performance of this module may not guaranteed.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rsa = RSA_1024::new();
    /// let prime1 = U512::from_str_radix("9696CB197B606EE02CCA95643546AF6EBDB3D58EF0382F2BA46BAF9089490A5856AD6E6DC3169C7B1AE4E6CDBB7B18BC9CFABDCCB5157649D475B3396A893B59", 16).unwrap();
    /// let prime2 = U512::from_str_radix("8821AE888CFE44FB3667C54A1C40452D02309B64940AE5FA957390F250BDC919DC350E4DB6B4E5CF05F393D9B4DF89E55BB5F7DFC114F465A250EF55284BF793", 16).unwrap();
    /// 
    /// rsa.calculate_keys(prime1.into_biguint(), prime2.into_biguint());
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.calculate_keys)
    pub fn calculate_keys(&mut self, prime_1: BigUInt<T, N>, prime_2: BigUInt<T, N>)
    {
        self.modulo = prime_1.wrapping_mul(&prime_2);
        let phi = prime_1.wrapping_sub_uint(1_u8).wrapping_mul(&prime_2.wrapping_sub_uint(1_u8));
        self.key_public = BigUInt::<T, N>::from_uint(2_u8);
        let mut one: BigUInt<T, N>;
        (one, self.key_private, _) = self.key_public.extended_gcd(&phi);

        while !one.is_one()
        {
            self.key_public.wrapping_add_assign_uint(1_u8);
            (one, self.key_private, _) = self.key_public.extended_gcd(&phi);
        }
        if self.key_private.is_underflow()
            { self.key_private = phi.wrapping_sub(&BigUInt::<T, N>::zero().wrapping_sub(&self.key_private)).wrapping_rem(&phi); }
        else
            { self.key_private.wrapping_rem_assign(&phi); }
    }

    // pub fn encrypt_biguint(&self, message: &BigUInt<[T, N>) -> BigUInt<T, N>
    /// Encrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is the plaintext to be encrypted in the forma of the type
    /// `&BigUInt<T, N>`, and should be less than `self.modulo` which is the
    /// product of the two prime numbers that were used to generate keys.
    ///
    /// # Output
    /// This method returns the encrypted data in the form of `BigUInt<T, N>`.
    /// 
    /// # Feature
    /// If `message` is greater than or equal to `self.modulo`,
    /// `message` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let public_key = U1024::from(7_u8);
    /// let private_key = U1024::from_str_radix("11A086687252F0D0ECE2E723F06808EEA5467A8EE1025EE5CAC299448454B33D1A9A97596144EBF52706CEEA13F427DF7000E05542DC34ABA46ECE12D1CF5D1286A9DCDE69407D78F699CF592CAFDF5741019FCA261BAA65529AB2FADC9D00F5712A14480C9724DA29C2354DB2DCABEDD89735A6B663984831B427061704E6D7", 16).unwrap();
    /// let modulo = U1024::from_str_radix("7b63acdb204495b67a3451fb92d83e8684ed59e8271098488b5230df9e50e6abba3a2371a8e273b4112fa8668bad171c10062254d40570b17f07a283bcab8b83252bdf633cc22120ec097748bc5f46e1492f2f4e4bf6ae1f1d88c0d5fdfea18474a7ba71b2e25f4624dbe6f8dac0f436092a8b375bf4816a231fac86564a9513", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulo.into_biguint());
    /// let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();
    /// 
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// println!("RSA_1024: Message = {}", message);
    /// 
    /// let cipher = rsa.encrypt_biguint(&message.into_biguint());
    /// println!("RSA_1024: Cipher = {}", cipher);
    /// assert_eq!(cipher.to_string(), "18990444273600905180995060516989662905254261731195035701565206356299504237028371798240343074431853089671262075536405296229684167693688757404552244758551778537810033711541966014458437636847820750518338546544929478474442855272138237274277632124436569414527899485686220502485118890982309705646418471801278267757");
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.encrypt_biguint)
    /// 
    /// # Example A of the failure for Message > self.modulo
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let public_key = U1024::from(5_u8);
    /// let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    /// let modulo = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.clone(), private_key.clone(), modulo.clone());
    /// 
    /// let message = modulo.wrapping_add_uint(1_u8);
    /// let distorted = U1024::one();
    /// 
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// println!("RSA_1024: Message = {}", message);
    /// println!("RSA_1024: Distorted = {}", distorted);
    /// 
    /// let cipher = rsa.encrypt_biguint(&message);
    /// println!("RSA_1024: Cipher = {}", cipher);
    /// let cypher = rsa.encrypt_biguint(&distorted);
    /// println!("RSA_1024: Cypher = {}", cypher);
    /// assert_eq!(cipher.to_string(), "1");
    /// assert_eq!(cypher.to_string(), "1");
    /// assert_eq!(cipher, cypher);
    /// 
    /// let recovered = rsa.decrypt_biguint(&cipher);
    /// println!("RSA_1024: Recovered = {}", recovered);
    /// assert_ne!(recovered, message);
    /// let back = rsa.decrypt_biguint(&cypher);
    /// println!("RSA_1024: Back = {}", back);
    /// assert_ne!(back, message);
    /// assert_eq!(back, distorted);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.encrypt_biguint)
    #[inline]
    pub fn encrypt_biguint(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        message.modular_pow(&self.key_public, &self.modulo)
    }

    // pub fn decrypt_biguint(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Decrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is the ciphertext to be decrypted in the form of the type
    /// `&BigUInt<T, N>`, and should be less than `self.modulo` which is the
    /// product of the two prime numbers that were used to generate keys.
    ///
    /// # Output
    /// This method returns the decrypted data in the form of `BigUInt<T, N>`.
    /// 
    /// # Feature
    /// If `cipher` is greater than or equal to `self.modulo`,
    /// `cipher` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// If `cipher` is the result of correct encryption,
    /// `cipher` can be neither greater than nor equal to `self.modulo`.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let public_key = U1024::from(5_u8);
    /// let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    /// let modulo = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulo.into_biguint());
    /// let message = U1024::from_string("7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777").unwrap();
    /// 
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// println!("RSA_1024: Message = {}", message);
    /// 
    /// let cipher = rsa.encrypt_biguint(&message.into_biguint());
    /// println!("RSA_1024: Cipher = {}", cipher);
    /// assert_eq!(cipher.to_string(), "14606510048253867029240420178508564108451783358298159314675299406727811512421137371598262884039995862707112191947061756986450584374674399814383778098601569518162693028818693718348755996275446231182729645715458251628617879799341412409848928638803776786415819193665392421517724782387829698801381262832285920668");
    /// 
    /// let recovered = rsa.decrypt_biguint(&cipher);
    /// println!("RSA_1024: Recovered = {}", recovered);
    /// assert_eq!(recovered, message.into_biguint());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.decrypt_biguint)
    #[inline]
    pub fn decrypt_biguint(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        cipher.modular_pow(&self.key_private, &self.modulo)
    }

    // pub fn encrypt_array_biguint<const M: usize>(&self, message: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    /// Encrypts data in the form of the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is the plaintext to be encrypted in the form of the type
    /// `&[BigUInt<T, N>; M]`, and each element of the array should be less
    /// than `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys.
    ///
    /// # Output
    /// This method returns the encrypted data
    /// in the form of the array of `BigUInt<T, N>`.
    /// 
    /// # Features
    /// If any element of `message` is greater than or equal to `self.modulo`,
    /// `message` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// 
    /// # Example 1 for RSA_1024
    /// click [here](struct@RSA_Generic#method.decrypt_array_biguint)
    ///
    /// # Notice
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
    /// Decrypts data in the form of the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is the ciphertext to be decrypted in the form of the type
    /// `&[BigUInt<T, N>; M]`, and each element of the array should be less
    /// than `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys.
    ///
    /// # Output
    /// This method returns the decrypted data
    /// in the form of the array of `BigUInt<T, N>`.
    /// 
    /// # Features
    /// If any element of `cipher` is greater than or equal to `self.modulo`,
    /// `cipher` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let public_key = U1024::from(5_u8);
    /// let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    /// let modulo = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulo.into_biguint());
    /// let message = [U1024::from_string("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap(),
    ///                 U1024::from_string("2222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222").unwrap(),
    ///                 U1024::from_string("3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333").unwrap()];
    /// 
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// println!("RSA_1024: Message = {}-{}-{}", message[0], message[1], message[2]);
    /// 
    /// let cipher = rsa.encrypt_array_biguint(&[message[0].into_biguint(), message[1].into_biguint(), message[2].into_biguint()]);
    /// println!("RSA_1024: Cipher = {}-{}-{}", cipher[0], cipher[1], cipher[2]);
    /// assert_eq!(cipher[0].to_string(), "11498347717307207939984362127351227343891647828020606575311543799151890764127473045317534512340185778744636841280042786182569650057937658603208825508942688973104517576288781331746748391854265838779283687892786769606159370940757555192222397126885152499974714907155577213949301954426519701752245072900923522752");
    /// assert_eq!(cipher[1].to_string(), "45166296929219312241602148452388101165982942196797433986896281306258443882182670297688602803542744940710777118285870195236223805296014729404603886413057185221555925116647357044154437182045242075682782556942753834770554330814409546713579993834957473055501649369674260085089688894770909747300380931410305465861");
    /// assert_eq!(cipher[2].to_string(), "104258245100557014100388000089255129244422186376824260941762474305558950933838723741556706570805144408966737408754572353981049992762264824762423516397166237816159126666560150516613931243159392446246807361060328409075514311188817416397236549955030332963112158611270173312280594534996614952700041036430688843711");
    /// 
    /// let recovered = rsa.decrypt_array_biguint(&cipher);
    /// println!("RSA_1024: Recovered = {}-{}-{}", recovered[0], recovered[1], recovered[2]);
    /// assert_eq!(recovered[0], message[0].into_biguint());
    /// assert_eq!(recovered[1], message[1].into_biguint());
    /// assert_eq!(recovered[2], message[2].into_biguint());
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.decrypt_array_biguint)
    ///
    /// # Notice
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
    /// Encrypts data in the form of the array of `T`.
    ///
    /// # Arguments
    /// `message` is the plaintext to be encrypted of the type `&[T; N]`, and
    /// `BigUInt::<T, N>::from_array(message).to_be()` should be less than
    /// `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys.
    ///
    /// # Output
    /// This method returns the encrypted data in the form of the array of `T`.
    /// 
    /// # Feature
    /// If `BigUInt::<T, N>::from_array(message).to_be()` is greater than or
    /// equal to `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys, `message` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// 
    /// # Example 1 for RSA_1024
    /// click [here](struct@RSA_Generic#method.decrypt_unit)
    pub fn encrypt_unit(&self, message: &[T; N]) -> [T; N]
    {
        let mut m = BigUInt::<T, N>::from_array(message.clone());
        m.to_be_assign();
        let mut c = self.encrypt_biguint(&m);
        c.to_be_assign();
        *c.get_number()
    }

    // pub fn decrypt_unit(&self, cipher: &[T; N]) -> [T; N]
    /// Decrypts data in the form of the array of `T`.
    ///
    /// # Arguments
    /// `cipher` is the ciphertext to be decrypted of the type `&[T; N]`, and
    /// `BigUInt::<T, N>::from_array(cipher).to_be()` should be less than
    /// `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys.
    ///
    /// # Output
    /// This method returns the decrypted data in the form of the array of `T`.
    /// 
    /// # Feature
    /// If `BigUInt::<T, N>::from_array(cipher).to_be()` is greater than or
    /// equal to `self.modulo` which is the product of the two prime numbers
    /// that were used to generate keys, `cipher` will be distorted
    /// so that the output of this method may not meaningful anymore.
    /// If `cipher` is the result of correct encryption,
    /// `BigUInt::<T, N>::from_array(cipher).to_be()` can be neither greater
    /// than nor equal to `self.modulo` which is the product of the two prime
    /// numbers that were used to generate keys.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use std::ptr::copy_nonoverlapping;
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let public_key = U1024::from(5_u8);
    /// let private_key = U1024::from_str_radix("3D4990127949DDB062F2BE417E8EACAB79F3215C306217A0C5974FEE15D4CB6D9348A161523F49F83D1CD49CB261C98259F04FECED08E08F3F0C5EF1FE695A291782AA0B9500911299CDAE0297337E9CB219F71411133C4440184C349FAC497EE809ED6D1B472409AB88A99B843FD61DBBBBC4C9686871D5221D137F89AF64CD", 16).unwrap();
    /// let modulo = U1024::from_str_radix("9937e82e2f38aa38f75edba3bc64afacb0dfd36678f53b11edfa47d33693fc91f03593734d9e38ec98c81387bdf477c5e0d8c7d0509631661d9eed5cfc07616849dec988a75bd976cd83c7b6b38cb3573d776435a28b33f2dbbcebb09e693d911fff63ff88e4de8c730a5ee8023b1c6e18a1551e949ebca6b1fedeff1dec08a9", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulo.into_biguint());
    /// let message = [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF];
    /// 
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// print!("RSA_1024: Message = [");
    /// for m in message
    ///     { print!("{:#X}, ", m); }
    /// println!("]");
    /// 
    /// let mut m = [0u32; 32];
    /// unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr(), 32); }
    /// let cipher = rsa.encrypt_unit(&m);
    /// print!("RSA_1024: Cipher = [");
    /// for c in cipher
    ///     { print!("{:#X}, ", c); }
    /// println!("]");
    /// assert_eq!(cipher, [0x5876D910, 0x9DF1BA1, 0x3D2ABEA9, 0xF9EAF9F1, 0xB4DD00B4, 0x4238994, 0x946574F8, 0xFB00D2B7, 0xD3F9E91D, 0x26D78E8F, 0x4D1B93C, 0x7666C3CC, 0x492EA323, 0xC7EFA926, 0x95E9D5CE, 0xF32C4732, 0x748103D, 0x298576A7, 0x4342BA6D, 0xFF59D1A9, 0x9D1585DD, 0xFA9B236F, 0x7B7982E2, 0x2B80425C, 0x1112A9DA, 0xA5EA8BC7, 0x41AF7FBB, 0x557B6333, 0xFA12D78B, 0x34B8451C, 0x867DF90A, 0xB412E61E]);
    /// 
    /// let recovered = rsa.decrypt_unit(&cipher);
    /// let mut rr = [0u128; 8];
    /// unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rr.as_mut_ptr() as *mut u32, 32); }
    /// 
    /// print!("RSA_1024: Recovered = [");
    /// for r in rr
    ///     { println!("{:#X}, ", r); }
    /// assert_eq!(rr, [0x_123456789ABCDEF00FEDCBA987654321, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF]);
    /// assert_eq!(rr, message);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.decrypt_unit)
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
    /// Encrypts data in the form of the array of `[T; N]`.
    ///
    /// # Arguments
    /// `message` is the plaintext to be encrypted of the type `&[[T; N]; M]`.
    ///
    /// # Output
    /// This method returns the encrypted data
    /// in the form of the array of `[T; N]`.
    /// 
    /// # Example 1 for RSA_1024
    /// click [here](struct@RSA_Generic#method.decrypt_array_unit)
    ///
    /// # Notice
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
    /// Decrypts data in the form of the array of `[T; N]`.
    ///
    /// # Arguments
    /// `cipher` is the ciphertext to be decrypted of the type `&[[T; N]; M]`.
    ///
    /// # Output
    /// This method returns the decrypted data
    /// in the form of the array of `[T; N]`.
    /// 
    /// # Example 1 for RSA_1024
    /// ```
    /// use std::ptr::copy_nonoverlapping;
    /// use cryptocol::asymmetric::RSA_1024;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let public_key = U1024::from(7_u8);
    /// let private_key = U1024::from_str_radix("4703E575111E5E33F674DB8CB5C7AA883BCBC715FFF564645CD67F2AB09470D71575D6D88FBB6BC0FABD4837B2F1F3F01FE4F7D135EF2FA15476D88107881CB8D6594A0010F987144DD6243268D07A6C7002F5949E0886BA36F8BAA886B0D8311277977315FC7F93CD95AB72592F65A2AB7BE609C69AFC3D9B54BA3BB78FAD87", 16).unwrap();
    /// let modulo = U1024::from_str_radix("636bdad717f750af25d6ccf831b121f1ed507d1eccbdf2f2e85f7ed55d9c9df9ead82cc8c93996daf8a2984dfa85ef1cf973c158184edc48430cc8b4a424f504093508b4a1235839fd887c0ef40d7740b770a375457b0e95cda768e23799f94c34982315e6974fa1e1fb63d2a1be2ed341f22275bd098a6bb56e7efe2ba6f2ef", 16).unwrap();
    /// let rsa = RSA_1024::new_with_keys(public_key.into_biguint(), private_key.into_biguint(), modulo.into_biguint());
    /// let message = [ [0x_123456789ABCDEF00FEDCBA987654321_u128, 0x_11223344556677889900AABBCCDDEEFF, 0x_FFEEDDCCBBAA00998877665544332211, 0x_1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x_11112222333344445555666677778888, 0x_99990000AAAABBBBCCCCDDDDEEEEFFFF],
    ///                 [0x_23456789ABCDEF00FEDCBA9876543211_u128, 0x_9900AABBCCDDEEFF1122334455667788, 0x_8877665544332211FFEEDDCCBBAA0099, 0x_9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0x_F1E2D3C4B5A6978879605A4B3C2D1F, 0x_98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x_55556666777788881111222233334444, 0x_CCCCDDDDEEEEFFFF99990000AAAABBBB],
    ///                 [0x_3456789ABCDEF00FEDCBA98765432112_u128, 0x_CCDDEEFF112233449900AABB55667788, 0x_44332211FFEEDD88776655CCBBAA0099, 0x_9807A6B5C5B6A70894D3E2F11F2E3D4C, 0x_F1E2D35A4B3C2D1FC4B5A697887960, 0x_4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x_55556666333344447777888811112222, 0x_EEEEFFFF99990000CCCCDDDDAAAABBBB] ];
    /// println!("RSA_1024: private key = {:X}:{:x}", rsa.get_private_key(), rsa.get_modulo());
    /// println!("RSA_1024: public key = {:X}:{:x}", rsa.get_public_key(), rsa.get_modulo());
    /// print!("RSA_1024: Message = [");
    /// for mm in message
    /// {
    ///     print!(" [");
    ///     for m in mm
    ///         { print!("{:#X}, ", m); }
    ///     print!(" ], ");
    /// }
    /// println!("]");
    /// 
    /// let mut m = [[0u32; 32]; 3];
    /// unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 32 * 3); }
    /// let cipher = rsa.encrypt_array_unit(&m);
    /// print!("RSA_1024: Cipher = [");
    /// for cc in cipher
    /// {
    ///     print!("[");
    ///     for c in cc
    ///         { print!("{:#X}, ", c); }
    ///     print!("], ");
    /// }
    /// println!("]");
    /// assert_eq!(cipher, [[0xE181EC1C, 0x86C53D7E, 0xC1EE84B, 0xC64EE28F, 0xBCB1272C, 0xDCCF7F8D, 0x5CCEE57E, 0xDEE7F806, 0x260E66E3, 0xEAF66FB4, 0x84C29D28, 0xF3331A75, 0x6A98F58E, 0xAF51309B, 0x988A192A, 0x36D16E3D, 0x8F7D9425, 0xD125283B, 0x64A9AC13, 0xBA293535, 0x48A937F6, 0x347A3932, 0x678E3AA9, 0x596C50E5, 0x372F32F3, 0x92CADF0A, 0x53A41E0, 0x527CDD75, 0x637C4679, 0x16575D1C, 0x1E452703, 0xC2A8AAFA],
    ///                     [0x6E971C27, 0x9D289F9E, 0x5890CBE1, 0xDCB604D0, 0x3DFF02E4, 0x8BB80EDF, 0x16025971, 0x47F83DFB, 0x15DB89ED, 0x37D3D81E, 0x1C784012, 0x96B5A8B2, 0x9B73CDEB, 0x19C3EB1C, 0x7380AF9B, 0xC7C05B11, 0x8778FFB1, 0xA4980451, 0x7817EF0E, 0xFBEC9BB0, 0x775226C3, 0x17DF9B7F, 0xBAD86876, 0xD482C2D8, 0x3AD54CCB, 0x63206A15, 0x5380238F, 0xCF893CDA, 0xD918FB53, 0x278CC36, 0x88966E4F, 0xED9F1A8A],
    ///                     [0x732F2B30, 0x4FF2B312, 0x764BC5FE, 0x1A83AAC2, 0x6B97E97A, 0x8740A298, 0xD0F1CCD, 0x6BE5488D, 0x55BCFAB6, 0x44D8DD33, 0xF0BDF5E3, 0xA4F2A20F, 0x5ED64723, 0xDFFEA2C7, 0xBB55E1E, 0xBA258D21, 0x75A9D8B8, 0x41087FE1, 0xF73A3142, 0x97551901, 0xB4534FEC, 0x94EAAEF3, 0x34A07F91, 0x3A31B17E, 0x6C49CF97, 0xAB7A50A1, 0xEAFD8E4, 0x3BDED4CB, 0x1360C56A, 0xA8053DEA, 0xF4967191, 0x9735C7FF]]);
    /// 
    /// let recovered = rsa.decrypt_array_unit(&cipher);
    /// let mut rrr = [[0u128; 8]; 3];
    /// unsafe { copy_nonoverlapping(recovered.as_ptr() as *const u32, rrr.as_mut_ptr() as *mut u32, 32 * 3); }
    /// 
    /// print!("RSA_1024: Recovered = [");
    /// for rr in rrr
    /// {
    ///     print!("[");
    ///     for r in rr
    ///         { print!("{:#X}, ", r); }
    ///     print!("], ");
    /// }
    /// println!("]");
    /// assert_eq!(rrr, [[0x123456789ABCDEF00FEDCBA987654321, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211, 0x1F2E3D4C5B6A70899807A6B5C4D3E2F1, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x102F3E4D5C6B7A8998A7B6C5D4E3F201, 0x11112222333344445555666677778888, 0x99990000AAAABBBBCCCCDDDDEEEEFFFF],
    ///                     [0x23456789ABCDEF00FEDCBA9876543211, 0x9900AABBCCDDEEFF1122334455667788, 0x8877665544332211FFEEDDCCBBAA0099, 0x9807A6B5C4D3E2F11F2E3D4C5B6A7089, 0xF1E2D3C4B5A6978879605A4B3C2D1F, 0x98A7B6C5D4E3F201102F3E4D5C6B7A89, 0x55556666777788881111222233334444, 0xCCCCDDDDEEEEFFFF99990000AAAABBBB],
    ///                     [0x3456789ABCDEF00FEDCBA98765432112, 0xCCDDEEFF112233449900AABB55667788, 0x44332211FFEEDD88776655CCBBAA0099, 0x9807A6B5C5B6A70894D3E2F11F2E3D4C, 0xF1E2D35A4B3C2D1FC4B5A697887960, 0x4E3F201102F3E4D98A7B6C5D5C6B7A89, 0x55556666333344447777888811112222, 0xEEEEFFFF99990000CCCCDDDDAAAABBBB]]);
    /// assert_eq!(rrr, message);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rsa_basic/struct.RSA_Generic.html#method.decrypt_array_unit)
    ///
    /// # Notice
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
