// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


macro_rules! pre_encrypt_into_vec {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {{
        let len = ($length_in_bytes + 1).next_multiple_of(Self::BLOCK_SIZE as u64) as usize / <$type>::size_in_bytes() as usize;
        $to.truncate(len);
        $to.resize(len + 1, <$type>::zero());
    }};
    // pre_encrypt_into_vec!(cipher, length_in_bytes, T);
    //
    // let mut len = if T::size_in_bytes() == 16 {16_usize} else {8};
    // len = (length_in_bytes + 1).next_multiple_of(Self::BLOCK_SIZE as u64) as usize / T::size_in_bytes();
    // cipher.truncate(len - 1);
    // cipher.resize(len, T::zero());
}
pub(super) use pre_encrypt_into_vec;

macro_rules! pre_decrypt_into_vec {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        $to.truncate(len - 1);
        $to.resize(len, <$type>::zero());
    };
}
pub(super) use pre_decrypt_into_vec;

macro_rules! pre_decrypt_into_vec_no_padding {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        $to.truncate(len);
        $to.resize(len, <$type>::zero());
    };
}
pub(super) use pre_decrypt_into_vec_no_padding;

macro_rules! pre_encrypt_into_array {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let mut len = if <$type>::size_in_bytes() == 16 {16_usize} else {8};
        len = ($length_in_bytes + 1).next_multiple_of(len as u64) as usize / <$type>::size_in_bytes() as usize;
        for i in len - 1..$to.len()
            { $to[i] = <$type>::zero(); }
    };
    // pre_encrypt_into_array!(cipher, length_in_bytes, T);
    //
    // let mut len = if T::size_in_bytes() == 16 {16_usize} else {8};
    // len = (length_in_bytes + 1).next_multiple_of(len as u64) as usize / T::size_in_bytes();
    // for i in len..M
    //     { cipher[i] = T::zero(); }
}
pub(super) use pre_encrypt_into_array;

macro_rules! pre_decrypt_into_array {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        for i in len - 1..$to.len()
            { $to[i] = <$type>::zero(); }
    };
}
pub(super) use pre_decrypt_into_array;

macro_rules! pre_decrypt_into_array_without_padding {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        for i in len..$to.len()
            { $to[i] = <$type>::zero(); }
    };
}
pub(super) use pre_decrypt_into_array_without_padding;



macro_rules! encrypt_into_array {
    ($me:expr, $iv:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        if ($length_in_bytes + 1).next_multiple_of(Self::BLOCK_SIZE as u64) > U::size_in_bytes() as u64 * N as u64
        {
            $me.set_failed();
            return 0;
        }
        pre_encrypt_into_array!($cipher, $length_in_bytes, $U);
        $me.encrypt($iv, $message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8)
    }};
    // if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
    // {
    //     self.set_failed();
    //     return 0;
    // }
    // pre_encrypt_into_array!(cipher, length_in_bytes, U);
    // self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)

    ($me:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        if ($length_in_bytes + 1).next_multiple_of(Self::BLOCK_SIZE as u64) > U::size_in_bytes() as u64 * N as u64
        {
            $me.set_failed();
            return 0;
        }
        pre_encrypt_into_array!($cipher, $length_in_bytes, $U);
        $me.encrypt($message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8)
    }};
}
pub(super) use encrypt_into_array;

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
        pre_encrypt_into_vec!($cipher, $length_in_bytes, $U);
        let len = $me.encrypt($message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8);
        $cipher.truncate(len as usize);
        len
    }};
}
pub(super) use encrypt_into_vec;

macro_rules! decrypt_into_array {
    ($me:expr, $iv:expr, $cipher:expr, $length_in_bytes:expr, $message:expr, $U:ty) => {{
        if $length_in_bytes as u128 - 1 > <$U>::size_in_bytes() as u128 * N as u128
        {
            $me.set_failed();
            return 0;
        }
        pre_decrypt_into_array!($message, $length_in_bytes, $U);
        $me.decrypt($iv, $cipher, $length_in_bytes, $message.as_mut_ptr() as *mut u8)
    }};
    // if length_in_bytes as u128 - 1 > U::size_in_bytes() as u128 * N as u128
    // {
    //     self.set_failed();
    //     return 0;
    // }
    // pre_decrypt_into_array!(message, length_in_bytes, U);
    // self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)

    ($me:expr, $cipher:expr, $length_in_bytes:expr, $message:expr, $U:ty) => {{
        if $length_in_bytes as u128 - 1 > <$U>::size_in_bytes() as u128 * N as u128
        {
            $me.set_failed();
            return 0;
        }
        pre_decrypt_into_array!($message, $length_in_bytes, $U);
        $me.decrypt($cipher, $length_in_bytes, $message.as_mut_ptr() as *mut u8)
    }};
}
pub(super) use decrypt_into_array;




macro_rules! encrypt_into_array_without_padding {
    ($me:expr, $iv:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        if $length_in_bytes as u128 > <$U>::size_in_bytes() as u128 * N as u128
        {
            $me.set_failed();
            return 0;
        }
        pre_encrypt_into_array!($cipher, $length_in_bytes, $U);
        $me.encrypt($iv, $message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8)
    }};
    // if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
    // {
    //     self.set_failed();
    //     return 0;
    // }
    // pre_encrypt_into_array!(cipher, length_in_bytes, U);
    // self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)

    ($me:expr, $message:expr, $length_in_bytes:expr, $cipher:expr, $U:ty) => {{
        if $length_in_bytes as u128 > <$U>::size_in_bytes() as u128 * N as u128
        {
            $me.set_failed();
            return 0;
        }
        pre_encrypt_into_array!($cipher, $length_in_bytes, $U);
        $me.encrypt($message, $length_in_bytes, $cipher.as_mut_ptr() as *mut u8)
    }};
}
pub(super) use encrypt_into_array_without_padding;

macro_rules! decrypt_into_array_without_padding {
    ($me:expr, $iv:expr, $cipher:expr, $length_in_bytes:expr, $message:expr, $U:ty) => {{
        if $length_in_bytes as u128 > <$U>::size_in_bytes() as u128 * N as u128
        {
            $me.set_failed();
            return 0;
        }
        pre_decrypt_into_array_without_padding!($message, $length_in_bytes, $U);
        $me.decrypt($iv, $cipher, $length_in_bytes, $message.as_mut_ptr() as *mut u8)
    }};
    // if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
    // {
    //     self.set_failed();
    //     return 0;
    // }
    // pre_decrypt_into_array!(message, length_in_bytes, U);
    // self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
}
pub(super) use decrypt_into_array_without_padding;


macro_rules! crypt_into_something_with_padding {
    ($type:ty) => {
        fn encrypt_into_array<U, const N: usize>(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_array!(self, iv, message, length_in_bytes, cipher, U)
        }

        fn encrypt_into_vec<U>(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_vec!(self, iv, message, length_in_bytes, cipher, U)
        }

        fn decrypt_into_array<U, const N: usize>(&mut self, iv: $type, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            decrypt_into_array!(self, iv, cipher, length_in_bytes, message, U)
        }
    };
}
pub(super) use crypt_into_something_with_padding;


macro_rules! crypt_into_something_with_padding_without_iv {
    () => {
        fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_array!(self, message, length_in_bytes, cipher, U)
        }

        fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_vec!(self, message, length_in_bytes, cipher, U)
        }

        fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            decrypt_into_array!(self, cipher, length_in_bytes, message, U)
        }
    };
}
pub(super) use crypt_into_something_with_padding_without_iv;


macro_rules! crypt_into_something_without_padding {
    ($type:ty) => {
        fn encrypt_into_array<U, const N: usize>(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_array_without_padding!(self, iv, message, length_in_bytes, cipher, U)
        }

        fn encrypt_into_vec<U>(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
        where U: SmallUInt + Copy + Clone
        {
            encrypt_into_vec!(self, iv, message, length_in_bytes, cipher, U)
        }

        fn decrypt_into_array<U, const N: usize>(&mut self, iv: $type, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
        where U: SmallUInt + Copy + Clone
        {
            decrypt_into_array_without_padding!(self, iv, cipher, length_in_bytes, message, U)
        }
    };
}
pub(super) use crypt_into_something_without_padding;