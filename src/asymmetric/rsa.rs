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

use std::ptr::{ copy_nonoverlapping, copy };

use crate::number::{ BigUInt, SmallUInt, IntUnion, LongUnion, LongerUnion, BigUInt_Prime, BigUInt_Modular };
use crate::random::Random;
use cryptocol::define_utypes_with;


struct RSA_Generic<const N: usize = 32, T = u32, const MR: usize = 5>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number:         BigUInt<T, N>,
    key_public:     BigUInt<T, N>,
    key_private:    BigUInt<T, N>,
    block:          [T; N],
}

impl<const N: usize, T, const MR: usize> RSA_Generic<N, T>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Prime = BigUInt<T, {N / 2}>;
    type Number = BigUInt<T, N>;

    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            number:         Number::new(),
            key_public:     Number::new(),
            key_private:    Number::new(),
            block:          [T; N],
        }
    }

    pub fn new_with_automatic_keys() -> Self
    {
        let mut rsa = Self
        {
            number:         Number::new(),
            key_public:     Number::new(),
            key_private:    Number::new(),
            block:          [T; N],
        };
        rsa.find_keys();
        rsa
    }

    #[inline]
    pub fn new_with_keys(key_public: BigUInt<T, N>, key_private: BigUInt<T, N>, number: BigUInt<T, N>) -> Self
    {
        Self
        {
            number,
            key_public,
            key_private,
            block: [T; N],
        }
    }

    #[inline]
    pub fn set_public_key(key_public: BigUInt<T, N>)
    {
        self.key_public = key_public;
    }

    #[inline]
    pub fn set_private_key(key_private: BigUInt<T, N>)
    {
        self.key_private = key_private;
    }

    #[inline]
    pub fn set_number(number: BigUInt<T, N>)
    {
        self.number = number;
    }
    
    fn choose_prime_numbers() -> (Prime, Prime)
    {
        let rand = Random::new();
        let prime_1: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(MR);
        let prime_2: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(MR);
        (prime_1, prime_2)
    }

    pub fn find_keys(&mut self)
    {
        let (prime_1, prime_2) = Self::choose_prime_numbers();
        self.number = prime_1.expanding_mul(&prime_2);
        let phi = prime_1.wrapping_sub_uint(1).expanding_mul(&prime_2.wrapping_sub_uint(1));
        self.key_public = Number::from_uint(2);
        let mut one: Number;
        (one, self.key_private, _) = self.key_public.extended_gcd(phi);
        while !one.is_one()
        {
            self.key_public.wrapping_add_uint(1);
            (one, self.key_private, _) = self.key_public.extended_gcd(phi);
        }
        if self.key_private.is_underflow()
            { self.key_private = self.number.wrapping_sub(Number::zero().wrapping_sub(&self.key_private)) }
        else
            { self.key_private.wrapping_rem_assign(&self.number); }
    }

    // pub fn encrypt_unit(&self, message: &BigUInt<[T, N>) -> BigUInt<T, N>
    /// Encrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is of the type `&BigUInt<T, N>`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the form of `BigUInt<T, N>`.
    #[inline]
    pub fn encrypt_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        message.modular_pow(&self.key_public, &number)
    }

    // pub fn decrypt_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Decrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&BigUInt<T, N>`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the form of `BigUInt<T, N>`.
    #[inline]
    pub fn decrypt_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        cipher.modular_pow(&self.key_private, &number)
    }

    // pub fn encrypt_array_unit<const M: usize>(&self, message: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    /// Encrypts data in the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `message` is of the type `&[BigUInt<T, N>; M]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data as the array of `BigUInt<T, N>`.
    pub fn encrypt_array_unit<const M: usize>(&self, message: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    {
        let mut cipher = [BigUInt::<T, N>::new(); M];
        for i in 0..M
            { cipher[i] = message[i].modular_pow(&self.key_public, &number); }
        cipher
    }

    // pub fn decrypt_array_unit(&self, cipher: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    /// Decrypts data in the array of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[BigUInt<T, N>; M]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data as the array of `BigUInt<T, N>`.
    #[inline]
    pub fn decrypt_array_unit(&self, cipher: &[BigUInt<T, N>; M]) -> [BigUInt<T, N>; M]
    {
        let mut message = [BigUInt::<T, N>::new(); M];
        for i in 0..M
            { message[i] = cipher[i].modular_pow(&self.key_private, &number); }
        message
    }

    // pub fn encrypt_chunck(&self, message: &[T; N]) -> [T; N]
    /// Encrypts data in the array of `T`.
    ///
    /// # Arguments
    /// `message` is of the type `&[T; N]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data as the array of `T`.
    ///
    /// # Counterpart methods
    /// For each trait
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    #[inline]
    pub fn encrypt_chunck(&self, message: &[T; N]) -> [T; N]
    {
        let mut m = BigUInt::<T, N>::from_array(message.clone());
        m.to_be_assign();
        let mut c = m.modular_pow(&self.key_public, &number);
        c.to_be_assign();
        *c.get_number()
    }

    // pub fn decrypt_chunk(&self, cipher: &[T; N]) -> [T; N]
    /// Decrypts data in the form of `BigUInt<T, N>`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&BigUInt<T, N>`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `BigUInt<T, N>`.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    #[inline]
    pub fn decrypt_chunk(&self, cipher: &[T; N]) -> [T; N]
    {
        let mut c = BigUInt::<T, N>::from_array(cipher.clone());
        c.to_be_assign();
        let mut m = c.modular_pow(&self.key_private, &number);
        m.to_be_assign();
        *m.get_number()
    }

    // pub fn encrypt_array_chunk<const M: usize>(&self, message: &[[T; N]; M]) -> [[T; N]; M]
    /// Encrypts data in the array of `[T; N]`.
    ///
    /// # Arguments
    /// `message` is of the type `&[[T; N]; M]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the array of `[T; N]`.
    ///
    /// # Counterpart methods
    /// For each trait
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    pub fn encrypt_array_chunk<const M: usize>(&self, message: &[[T; N]; M]) -> [[T; N]; M]
    {
        let mut cipher = [[T::zero(); N]; M];
        for i in 0..M
        {
            let mut m = BigUInt::<T, N>::from_array(message[i].clone());
            m.to_be_assign();
            let mut c = m.modular_pow(&self.key_public, &number);
            c.to_be_assign();
            cipher[i] = *c.get_number();
        }
        cipher
    }

    // pub fn decrypt_array_chunk(&self, cipher: &[[T; N]; M]) -> [[T; N]; M]
    /// Decrypts data in the array of `[T; N]`.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[[T; N]; M]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `[T; N]`.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    #[inline]
    pub fn decrypt_array_chunk(&self, cipher: &[[T; N]; M]) -> [[T; N]; M]
    {
        let mut message = [BigUInt::<T, N>::new(); M];
        for i in 0..M
        {
            let mut c = BigUInt::<T, N>::from_array(cipher[i].clone());
            c.to_be_assign();
            let mut m = c.modular_pow(&self.key_private, &number);
            m.to_be_assign();
             message[i] = *m.get_number();
        }
        message
    }

    // // pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    // /// 
    // #[inline]
    // pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     message.to_be().modular_pow(&self.key_private, &number).to_be()
    // }

    // // pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // ///
    // #[inline]
    // pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     cipher.to_be().modular_pow(&self.key_public, &number).to_be()
    // }

    // pub fn verify_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    // {
    //     cipher.to_be().modular_pow(&self.key_public, &number).to_be()
    // }
}
