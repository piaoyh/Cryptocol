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

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::{ BigUInt, BigUInt_Modular, BigUInt_Prime, SmallUInt };
use crate::random::Random;


pub type RSA_4096 = RSA_Generic<128, u32>;
pub type RSA_2048 = RSA_Generic<64, u32>;
pub type RSA_1024 = RSA_Generic<32, u32>;

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
/// # Why RSA?
/// 
/// 
/// # Short History of birth of RSA
/// 
/// 
/// # Use of RSA and its variants
/// 
/// 
/// # Generic Parameters
/// `N`: 
/// 
/// `T`: 
/// 
/// `MR`: 
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
/// `RSA_1024` in order to use official RSA as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::asymmetric::RSA_4096;
/// use cryptocol::asymmetric::RSA_2048;
/// use cryptocol::asymmetric::RSA_1024;
/// ```
/// 
/// # Notice for Practical Use
/// 
pub struct RSA_Generic<const N: usize, T, const MR: usize = 5>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    modulo: BigUInt<T, N>,
    key_public: BigUInt<T, N>,
    key_private: BigUInt<T, N>,
    block: [T; N],
}

impl<const N: usize, T, const MR: usize> RSA_Generic<N, T, MR>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
            block: [T::zero(); N],
        }
    }

    pub fn new_with_automatic_keys() -> Self
    {
        let mut rsa = Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
            block: [T::zero(); N],
        };
        rsa.find_keys();
        rsa
    }

    #[inline]
    pub fn new_with_keys(key_public: BigUInt<T, N>, key_private: BigUInt<T, N>, modulo: BigUInt<T, N>) -> Self
    {
        Self
        {
            modulo,
            key_public,
            key_private,
            block: [T::zero(); N],
        }
    }

    /// # Caution
    /// M should be N/2. Otherwise, the performance of this module may not
    /// guaranteed .
    pub fn new_with_primes(prime_1: BigUInt<T, N>, prime_2: BigUInt<T, N>) -> Self
    {
        let mut rsa = Self
        {
            modulo: BigUInt::<T, N>::new(),
            key_public: BigUInt::<T, N>::new(),
            key_private: BigUInt::<T, N>::new(),
            block: [T::zero(); N],
        };
        rsa.calculate_keys(prime_1, prime_2);
        rsa
    }

    /// # Caution
    /// M should be N/2. Otherwise, the performance of this module may not
    /// guaranteed .
    pub fn new_with_prepared_keys() -> Self
    {
        let length = T::size_in_bytes() as usize * N;
        let mut rand = Random::new();
        let prime_1 = rand.random_prime_with_half_length_using_prepared();
        let mut prime_2 = rand.random_prime_with_half_length_using_prepared();
        while prime_1 == prime_2
            { prime_2 = rand.random_prime_with_half_length_using_prepared(); }
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
        let prime_1 = rand.random_prime_with_half_length_using_miller_rabin_biguint(MR);
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
