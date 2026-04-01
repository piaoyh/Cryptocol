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


use std::ptr::copy_nonoverlapping;

use crate::number::SmallUInt;
use crate::random::Random;
use crate::asymmetric::{ OAEP, RSA_Generic, Hash };


macro_rules! pre_encrypt_into_array {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let mut len = if <$type>::size_in_bytes() == 16 {16_usize} else {8};
        len = ($length_in_bytes + 1).next_multiple_of(len as u64) as usize / <$type>::size_in_bytes() as usize;
        for i in len - 1..$to.len()
            { $to[i] = <$type>::zero(); }
    };
}

macro_rules! pre_encrypt_into_vec {
    ($to:expr, $type:ty) => {{
        let len = (T::size_in_bytes() as usize * N) / <$type>::size_in_bytes() as usize + 1;
        $to.truncate(len);
        $to.resize(len, <$type>::zero());
    }};
}

macro_rules! pre_decrypt_into_array {
    ($to:expr, $type:ty) => {
        for i in 0..$to.len()
            { $to[i] = <$type>::zero(); }
    };
}

macro_rules! pre_decrypt_into_vec {
    ($to:expr, $type:ty) => {
        let len = (T::size_in_bytes() as usize * N - 11) / <$type>::size_in_bytes() as usize + 1;
        $to.truncate(len);
        $to.resize(len, <$type>::zero());
    };
}

macro_rules! encrypt_into_array {
    ($me:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty, $M:expr) => {{
        if ($length_in_bytes > (T::size_in_bytes() as u64 * N as u64 - 11)) || ((<$U>::size_in_bytes() as u64 * M as u64) < (T::size_in_bytes() as u64 * N as u64))
            { return 0; }
        pre_encrypt_into_array!($cipher, $length_in_bytes, $U);
        $me.encrypt($message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8)
    }};
}

macro_rules! encrypt_into_vec {
    ($me:expr, $iv:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        pre_encrypt_into_vec!($cipher, $length_in_bytes, $U);
        let len = $me.encrypt($iv, $message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8);
        $cipher.truncate(len as usize);
        len
    }};
    // pre_encrypt_into_vec!(cipher, length_in_bytes, U);
    // let len = self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
    // cipher.truncate(len as usize);
    // len

    ($me:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        pre_encrypt_into_vec!($cipher, $U);
        let len = $me.encrypt($message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8);
        $cipher.truncate(len as usize);
        len
    }};
}

macro_rules! decrypt_into_array {
    ($me:expr, $cipher:expr, $message:expr, $U:ty) => {{
        pre_decrypt_into_array!($message, $U);
        $me.decrypt($cipher, $message.as_mut_ptr() as *mut u8)
    }};
}

macro_rules! crypt_into_something_with_padding {
    () => {
        fn encrypt_into_array<U, const M: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; M]) -> u64
        where U: SmallUInt
        {
            encrypt_into_array!(self, message, length_in_bytes, cipher, U, M)
        }

        fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
        where U: SmallUInt
        {
            encrypt_into_vec!(self, message, length_in_bytes, cipher, U)
        }

        fn decrypt_into_array<U, const M: usize>(&mut self, cipher: *const u8, message: &mut [U; M]) -> u64
        where U: SmallUInt
        {
            decrypt_into_array!(self, cipher, message, U)
        }

        fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
        where U: SmallUInt
        {
            pre_decrypt_into_vec!(message, U);
            let len = self.decrypt(cipher, message.as_mut_ptr() as *mut u8);
            let size = len as usize / U::size_in_bytes() as usize
                        + if len as usize % U::size_in_bytes() as usize == 0 {0} else {1};
            message.truncate(size);
            len
        }

        fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
        where U: SmallUInt, V: SmallUInt
        {
            pre_decrypt_into_vec!(message, V);
            self.decrypt_into_vec(cipher.as_ptr() as *const u8, message)
        }
    };
}



// fn mgf1<const L: usize, const M: usize, H: Hash>(seed: [u8; M], hash: &mut H) -> Option<[u8; L]>
/// Is Mask Generation Function 1 (MGF1).
/// 
/// # Arguments
/// - seed: is desirable that `S::size_in_bytes() * M` is greater than or
///   equal to the output length of the hash function used in this method,
///   and the security level is the best when `S::size_in_bytes() * M`
///   is equal to the output length of the hash function.
/// - hash: is hash function object.
fn mgf1<const L: usize, const M: usize, H: Hash>(seed: [u8; M], hash: &mut H) -> Option<[u8; L]>
{
    if M != hash.get_default_length_in_bytes()
        { return None; }

    let mut mask = [0u8; L];
    for counter in 0..L/M
    {
        let code = hash.calculate_hash_code(&seed, counter as u32);
        unsafe { copy_nonoverlapping(code.as_ptr(), mask.as_mut_ptr().add(counter * M), M); }
    }
    let code = hash.calculate_hash_code(&seed, (L / M) as u32);
    unsafe { copy_nonoverlapping(code.as_ptr(), mask.as_mut_ptr().add(L / M * M), L % M); }
    Some(mask)
}


impl<const N: usize, T, const MR: usize, HashType> OAEP<HashType> for RSA_Generic<N, T, MR, HashType>
where T: SmallUInt, HashType: Hash
{
    #[inline]
    fn set_hash(&mut self, hash: HashType)
    {
        self.set_hash(hash);
    }

    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let size = T::size_in_bytes() as usize * N;
        if length_in_bytes > size as u64 - 11
            { return 0; }

        let mut m = [T::zero(); N];
        let count = size - length_in_bytes as usize;
        unsafe {
            *((m.as_mut_ptr() as *mut u8).add(1)) = Self::BT;
            let mut any = Random::new();
            for i in 2..count-1
            {
                let mut r = any.random_u8();
                while r == 0
                    { r = any.random_u8(); }
                *((m.as_mut_ptr() as *mut u8).add(i)) = r;
            }
            let ptr = (m.as_mut_ptr() as *mut u8).add(count);
            copy_nonoverlapping(message, ptr, length_in_bytes as usize);
        }
        let c = self.encrypt_unit(&m);
        unsafe { copy_nonoverlapping(c.as_ptr() as *const u8, cipher, size); }
        size as u64
    }

    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64
    {
        let size = T::size_in_bytes() as usize * N;
        let mut c = [T::zero(); N];
        unsafe { copy_nonoverlapping(cipher, c.as_mut_ptr() as *mut u8, size); }
        let m = self.decrypt_unit(&c);
        let ptr = m.as_ptr() as *const u8;
        let mut len = 0_usize;
        unsafe {
            if (*ptr != 0) || (*(ptr.add(1)) != Self::BT)
                { return 0; }
            for i in 2..size
            {
                if *ptr.add(i) == 0
                    { len = i + 1; }
            }
            if len < 12
                { return 0; }
            copy_nonoverlapping(ptr.add(len), message, size - len);
        }
        (size - len) as u64
    }

    crypt_into_something_with_padding!{}
}

// impl<const N: usize, T, H, const MR: usize> RSA_Generic<N, T, H, MR>
// where T: SmallUInt, H: Hash
// {
//     pub(super) fn set_hash(&mut self, hash: H)
//     {
//         self.hash = hash;
//     }
// }