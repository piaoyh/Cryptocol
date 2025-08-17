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



macro_rules! crypt_ecb_with_padding_pkcs7 {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty, $initial:expr) => {
        fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut encoded: u64;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }
            
            block = 0;
            let mut block_union = <$container>::new_with($initial);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                let padding = (Self::BLOCK_SIZE - tail) as u8;
                block_union.set(block);
                for i in tail..Self::BLOCK_SIZE
                    { block_union.set_ubyte_(i, padding); }
            }
            encoded = $func_enc(self, block_union.get());
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut decoded: $type;
            if length_in_bytes > Self::BLOCK_SIZE as u64
            {
                for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
                {
                    unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                    decoded = $func_dec(self, block);
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                    progress += Self::BLOCK_SIZE as u64;
                }
            }

            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            let padding_bytes = decoded_union.get_ubyte_(MASK);
            if padding_bytes > Self::BLOCK_SIZE as u8
            {
                self.set_failed();
                return 0;
            }
            let message_bytes = Self::BLOCK_SIZE - padding_bytes as usize;
            for i in message_bytes..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(i) != padding_bytes
                {
                    self.set_failed();
                    return 0;
                }
            }
            unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
            self.set_successful();
            progress + message_bytes as u64
        }
    };
    (u64) => { crypt_ecb_with_padding_pkcs7!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion, 0x_08_08_08_08__08_08_08_08} };
    (u128) => { crypt_ecb_with_padding_pkcs7!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion, 0x_08_08_08_08__08_08_08_08__08_08_08_08__08_08_08_08} };
}
pub(super) use crypt_ecb_with_padding_pkcs7;


macro_rules! crypt_ecb_with_padding_iso {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty) => {
        fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut encoded: $type;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            block = 0;
            let mut block_union = <$container>::new_with(0b_1000_0000);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                block_union.set(block);
                block_union.set_ubyte_(tail, 0b_1000_0000);
            }
            encoded = $func_enc(self, block_union.get());
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut decoded: $type;
            if length_in_bytes > 8
            {
                for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
                {
                    unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                    decoded = $func_dec(self, block);
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                    progress += Self::BLOCK_SIZE as u64;
                }
            }
            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            for i in 0..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(MASK - i) == 0
                    { continue; }
                if decoded_union.get_ubyte_(MASK - i) == 0b_1000_0000_u8
                {
                    let message_bytes = MASK - i;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
                    self.set_successful();
                    return progress + message_bytes as u64;
                }
                else
                {
                    self.set_failed();
                    return 0;
                }
            }
            self.set_failed();
            return 0;
        }
    };
    (u64) => { crypt_ecb_with_padding_iso!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion} };
    (u128) => { crypt_ecb_with_padding_iso!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion} };
}
pub(super) use crypt_ecb_with_padding_iso;


macro_rules! crypt_cbc_with_padding_pkcs7 {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty, $initial:expr) => {
        fn encrypt(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut encoded = iv;
            let mut block: $type = 0;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block ^ encoded);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }
            
            block = 0;
            let mut block_union = <$container>::new_with($initial);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                let padding = (Self::BLOCK_SIZE - tail) as u8;
                block_union.set(block);
                for i in tail..Self::BLOCK_SIZE
                    { block_union.set_ubyte_(i, padding); }
            }
            encoded = $func_enc(self, block_union.get() ^ encoded);
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, mut iv: $type, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut decoded: $type;
            let mut block: $type = 0;
            if length_in_bytes > Self::BLOCK_SIZE as u64
            {
                for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
                {
                    unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                    decoded = iv ^ $func_dec(self, block);
                    iv = block;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                    progress += Self::BLOCK_SIZE as u64;
                }
            }

            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = iv ^ $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            let padding_bytes = decoded_union.get_ubyte_(MASK);
            if padding_bytes > Self::BLOCK_SIZE as u8
            {
                self.set_failed();
                return 0;
            }
            let message_bytes = Self::BLOCK_SIZE - padding_bytes as usize;
            for i in message_bytes..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(i) != padding_bytes
                {
                    self.set_failed();
                    return 0;
                }
            }
            unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
            self.set_successful();
            progress + message_bytes as u64
        }
    };
    (u64) => { crypt_cbc_with_padding_pkcs7!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion, 0x_08_08_08_08__08_08_08_08} };
    (u128) => { crypt_cbc_with_padding_pkcs7!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion, 0x_08_08_08_08__08_08_08_08__08_08_08_08__08_08_08_08} };
}
pub(super) use crypt_cbc_with_padding_pkcs7;


macro_rules! crypt_cbc_with_padding_iso {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty) => {
        fn encrypt(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut encoded = iv;
            let mut block: $type = 0;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block ^ encoded);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }
            block = 0;
            let mut block_union = <$container>::new_with(0b_1000_0000);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                block_union.set(block);
                block_union.set_ubyte_(tail, 0b_1000_0000);
            }
            encoded = $func_enc(self, block_union.get() ^ encoded);
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, mut iv: $type, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut decoded: $type;
            let mut block: $type = 0;
            for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                decoded = iv ^ $func_dec(self, block);
                iv = block;
                unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = iv ^ $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            for i in 0..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(MASK - i) == 0
                    { continue; }
                if decoded_union.get_ubyte_(MASK - i) == 0b_1000_0000_u8
                {
                    let message_bytes = MASK - i;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
                    self.set_successful();
                    return progress + message_bytes as u64;
                }
                else
                {
                    self.set_failed();
                    return 0;
                }
            }
            self.set_failed();
            return 0;
        }
    };
    (u64) => { crypt_cbc_with_padding_iso!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion} };
    (u128) => { crypt_cbc_with_padding_iso!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion} };
}
pub(super) use crypt_cbc_with_padding_iso;


macro_rules! crypt_pcbc_with_padding_pkcs7 {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty, $initial:expr) => {
        fn encrypt(&mut self, mut iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut encoded: $type;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block ^ iv);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                iv = block ^ encoded;
                progress += Self::BLOCK_SIZE as u64;
            }
            
            block = 0;
            let mut block_union = <$container>::new_with($initial);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                let padding = (Self::BLOCK_SIZE - tail) as u8;
                block_union.set(block);
                for i in tail..Self::BLOCK_SIZE
                    { block_union.set_ubyte_(i, padding); }
            }
            encoded = $func_enc(self, block_union.get() ^ iv);
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, mut iv: $type, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut decoded: $type;
            if length_in_bytes > Self::BLOCK_SIZE as u64
            {
                for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
                {
                    unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                    decoded = iv ^ $func_dec(self, block);
                    iv = block ^ decoded;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                    progress += Self::BLOCK_SIZE as u64;
                }
            }

            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = iv ^ $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            let padding_bytes = decoded_union.get_ubyte_(MASK);
            let message_bytes = Self::BLOCK_SIZE - padding_bytes as usize;
            for i in message_bytes..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(i) != padding_bytes
                {
                    self.set_failed();
                    return 0;
                }
            }
            unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
            self.set_successful();
            progress + message_bytes as u64
        }
    };
    (u64) => { crypt_pcbc_with_padding_pkcs7!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion, 0x_08_08_08_08__08_08_08_08} };
    (u128) => { crypt_pcbc_with_padding_pkcs7!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion, 0x_08_08_08_08__08_08_08_08__08_08_08_08__08_08_08_08} };
}
pub(super) use crypt_pcbc_with_padding_pkcs7;


macro_rules! crypt_pcbc_with_padding_iso {
    ($type:ty, $func_enc:expr, $func_dec:expr, $container:ty) => {
        fn encrypt(&mut self, mut iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut encoded: $type;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = $func_enc(self, block ^ iv);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                iv = block ^ encoded;
                progress += Self::BLOCK_SIZE as u64;
            }

            block = 0;
            let mut block_union = <$container>::new_with(0b_1000_0000);
            if progress != length_in_bytes
            {
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                block_union.set(block);
                block_union.set_ubyte_(tail, 0b_1000_0000);
            }

            encoded = $func_enc(self, block_union.get() ^ iv);
            unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            self.set_successful();
            progress + Self::BLOCK_SIZE as u64
        }

        fn decrypt(&mut self, mut iv: $type, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes & MASK as u64 != 0)
            {
                self.set_failed();
                return 0;
            }
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut decoded: $type;
            if length_in_bytes > Self::BLOCK_SIZE as u64
            {
                for _ in 0..(length_in_bytes >> SHR) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
                {
                    unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                    decoded = iv ^ $func_dec(self, block);
                    iv = block ^ decoded;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                    progress += Self::BLOCK_SIZE as u64;
                }
            }

            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
            decoded = iv ^ $func_dec(self, block);
            let decoded_union = <$container>::new_with(decoded);
            for i in 0..Self::BLOCK_SIZE
            {
                if decoded_union.get_ubyte_(MASK - i) == 0
                    { continue; }
                if decoded_union.get_ubyte_(MASK - i) == 0b_1000_0000_u8
                {
                    let message_bytes = MASK - i;
                    unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), message_bytes); }
                    self.set_successful();
                    return progress + message_bytes as u64;
                }
                else
                {
                    self.set_failed();
                    return 0;
                }
            }
            self.set_failed();
            return 0;
        }
    };
    (u64) => { crypt_pcbc_with_padding_iso!{u64, Self::encrypt_u64, Self::decrypt_u64, LongUnion} };
    (u128) => { crypt_pcbc_with_padding_iso!{u128, Self::encrypt_u128, Self::decrypt_u128, LongerUnion} };
}
pub(super) use crypt_pcbc_with_padding_iso;


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


macro_rules! crypt_cfb {
    ($type:ty, $func:expr) => {
        fn encrypt(&mut self, iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut encoded = iv;
            let mut block: $type = 0;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                encoded = block ^ $func(self, encoded);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            if progress == length_in_bytes
            {
                self.set_successful();
                progress
            }
            else
            {
                block = 0;
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                encoded = block ^ $func(self, encoded);
                unsafe { copy_nonoverlapping(&encoded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                self.set_successful();
                progress + tail as u64
            }
        }

        fn decrypt(&mut self, mut iv: $type, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            #[allow(non_snake_case)]    let MASK = Self::BLOCK_SIZE - 1;
            let mut progress = 0_u64;
            let mut decoded: $type;
            let mut block: $type = 0;
            for _ in 0..(length_in_bytes >> SHR)  // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                decoded = block ^ $func(self, iv);
                iv = block;
                unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            if progress == length_in_bytes
            {
                progress
            }
            else
            {
                block = 0;
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { cipher.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                decoded = block ^ $func(self, iv);
                unsafe { copy_nonoverlapping(&decoded as *const $type as *const u8, message.add(progress as usize), tail); }
                self.set_successful();
                progress + tail as u64
            }
        }
    };
    (u64) => { crypt_cfb!{u64, Self::encrypt_u64} };
    (u128) => { crypt_cfb!{u128, Self::encrypt_u128} };
}
pub(super) use crypt_cfb;


macro_rules! crypt_ofb {
    ($type:ty, $func:expr) => {
        fn encrypt(&mut self, mut iv: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut coded: $type;
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                iv = $func(self, iv);
                coded = block ^ iv;
                unsafe { copy_nonoverlapping(&coded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            if progress == length_in_bytes
            {
                self.set_successful();
                progress
            }
            else
            {
                block = 0;
                let tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
                coded = block ^ $func(self, iv);
                unsafe { copy_nonoverlapping(&coded as *const $type as *const u8, cipher.add(progress as usize), tail); }
                self.set_successful();
                progress + tail as u64
            }
        }
    };
    (u64) => { crypt_ofb!{u64, Self::encrypt_u64} };
    (u128) => { crypt_ofb!{u128, Self::encrypt_u128} };
}
pub(super) use crypt_ofb;


macro_rules! crypt_ctr {
    ($type:ty, $func:expr) => {
        fn encrypt(&mut self, mut nonce: $type, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
        {
            #[allow(non_snake_case)]    let SHR = Self::BLOCK_SIZE.ilog2();
            let mut progress = 0_u64;
            let mut block: $type = 0;
            let mut coded: $type;
            nonce = nonce.wrapping_add(1);
            for _ in 0..length_in_bytes >> SHR    // length_in_bytes >> 4 == length_in_bytes / 16
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut $type as *mut u8, Self::BLOCK_SIZE); }
                coded = block ^ $func(self, nonce);
                nonce = nonce.wrapping_add(1);
                unsafe { copy_nonoverlapping(&coded as *const $type as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
                progress += Self::BLOCK_SIZE as u64;
            }

            let mut tail = Self::BLOCK_SIZE;
            if progress + 8 == length_in_bytes
            {
                unsafe { copy_nonoverlapping(message.add(progress as usize - Self::BLOCK_SIZE) as *const u8, (&mut block) as *mut u64 as *mut u8, Self::BLOCK_SIZE); }
            }
            else
            {
                block = 0;
                tail = (length_in_bytes - progress) as usize;
                let addr = unsafe { message.add(progress as usize) as *const u8 };
                unsafe { copy_nonoverlapping(addr, &mut block as *mut $type as *mut u8, tail); }
            }
            coded = block ^ $func(self, nonce);
            unsafe { copy_nonoverlapping(&coded as *const $type as *const u8, cipher.add(progress as usize), tail); }
            self.set_successful();
            progress + tail as u64
        }
    };
    (u64) => { crypt_ctr!{u64, Self::encrypt_u64} };
    (u128) => { crypt_ctr!{u128, Self::encrypt_u128} };
}
pub(super) use crypt_ctr;


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
