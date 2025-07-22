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


use crate::number::{IntUnion, LongUnion};
use crate::symmetric::{ SmallCryptor,  Rijndael_64_64 };


impl SmallCryptor<u64, 8> for Rijndael_64_64
{
    #[inline] fn set_key(&mut self, key: [u8; 8])   { self.set_key(&key); }

    fn set_key_unit(&mut self, key: u64)
    {
        let long = LongUnion::new_with(key);
        let k = [long.get_ubyte_(0), long.get_ubyte_(1), long.get_ubyte_(2), long.get_ubyte_(3),
                          long.get_ubyte_(4), long.get_ubyte_(5), long.get_ubyte_(6), long.get_ubyte_(7)];
        self.set_key(&k);
    }

    fn encrypt_unit(&mut self, message: u64) -> u64
    {
        let long = LongUnion::new_with(message);
        let m = [IntUnion::new_with(long.get_uint_(0)),
                                IntUnion::new_with(long.get_uint_(1))];
        let res = self._encrypt(&m);
        LongUnion::new_with_uints([res[0].get(), res[1].get()]).get()
    }
    
    #[inline] fn decrypt_unit(&mut self, cipher: u64) -> u64
    {
        let long = LongUnion::new_with(cipher);
        let c = [IntUnion::new_with(long.get_uint_(0)),
                                IntUnion::new_with(long.get_uint_(1))];
        let res = self._decrypt(&c);
        LongUnion::new_with_uints([res[0].get(), res[1].get()]).get()
    }

    #[inline] fn turn_inverse(&mut self)    { self.turn_inverse(); }
    #[inline] fn turn_encryptor(&mut self)  { self.turn_encryptor(); }
    #[inline] fn turn_decryptor(&mut self)  { self.turn_decryptor(); }
    // #[inline] fn clone_cryptor(&self) -> SmallCryptor<u64, 8>   { self.clone() }
}