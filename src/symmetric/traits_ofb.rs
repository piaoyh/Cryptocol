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


use std::ptr::copy_nonoverlapping;
use std::vec::Vec;

use crate::number::SmallUInt;
use crate::symmetric::DES_Generic;
use crate::symmetric::{ des_pre_encrypt_into_vec, des_pre_encrypt_into_array,
                        des_pre_decrypt_into_vec, des_pre_decrypt_into_array };

pub trait OFB<T> : Sized
{
    fn encrypt(&mut self, iv: T, from: *const u8, length_in_bytes: u64, to: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data without padding.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_encrypt_into_vec!(cipher, length_in_bytes, U);
        let len = self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    #[inline]
    fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }



    #[inline]
    fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.encrypt(iv, cipher, length_in_bytes, message)
    }

    fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    #[inline]
    fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, iv: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }
}


impl <const ROUND: usize, const SHIFT: u128,
const PC101: u8, const PC102: u8, const PC103: u8, const PC104: u8,
const PC105: u8, const PC106: u8, const PC107: u8, const PC108: u8,
const PC109: u8, const PC110: u8, const PC111: u8, const PC112: u8,
const PC113: u8, const PC114: u8, const PC115: u8, const PC116: u8,
const PC117: u8, const PC118: u8, const PC119: u8, const PC120: u8,
const PC121: u8, const PC122: u8, const PC123: u8, const PC124: u8,
const PC125: u8, const PC126: u8, const PC127: u8, const PC128: u8,
const PC129: u8, const PC130: u8, const PC131: u8, const PC132: u8,
const PC133: u8, const PC134: u8, const PC135: u8, const PC136: u8,
const PC137: u8, const PC138: u8, const PC139: u8, const PC140: u8,
const PC141: u8, const PC142: u8, const PC143: u8, const PC144: u8,
const PC145: u8, const PC146: u8, const PC147: u8, const PC148: u8,
const PC149: u8, const PC150: u8, const PC151: u8, const PC152: u8,
const PC153: u8, const PC154: u8, const PC155: u8, const PC156: u8,
const PC201: u8, const PC202: u8, const PC203: u8, const PC204: u8,
const PC205: u8, const PC206: u8, const PC207: u8, const PC208: u8,
const PC209: u8, const PC210: u8, const PC211: u8, const PC212: u8,
const PC213: u8, const PC214: u8, const PC215: u8, const PC216: u8,
const PC217: u8, const PC218: u8, const PC219: u8, const PC220: u8,
const PC221: u8, const PC222: u8, const PC223: u8, const PC224: u8,
const PC225: u8, const PC226: u8, const PC227: u8, const PC228: u8,
const PC229: u8, const PC230: u8, const PC231: u8, const PC232: u8,
const PC233: u8, const PC234: u8, const PC235: u8, const PC236: u8,
const PC237: u8, const PC238: u8, const PC239: u8, const PC240: u8,
const PC241: u8, const PC242: u8, const PC243: u8, const PC244: u8,
const PC245: u8, const PC246: u8, const PC247: u8, const PC248: u8,
const IP01: u8, const IP02: u8, const IP03: u8, const IP04: u8,
const IP05: u8, const IP06: u8, const IP07: u8, const IP08: u8,
const IP09: u8, const IP10: u8, const IP11: u8, const IP12: u8,
const IP13: u8, const IP14: u8, const IP15: u8, const IP16: u8,
const IP17: u8, const IP18: u8, const IP19: u8, const IP20: u8,
const IP21: u8, const IP22: u8, const IP23: u8, const IP24: u8,
const IP25: u8, const IP26: u8, const IP27: u8, const IP28: u8,
const IP29: u8, const IP30: u8, const IP31: u8, const IP32: u8,
const IP33: u8, const IP34: u8, const IP35: u8, const IP36: u8,
const IP37: u8, const IP38: u8, const IP39: u8, const IP40: u8,
const IP41: u8, const IP42: u8, const IP43: u8, const IP44: u8,
const IP45: u8, const IP46: u8, const IP47: u8, const IP48: u8,
const IP49: u8, const IP50: u8, const IP51: u8, const IP52: u8,
const IP53: u8, const IP54: u8, const IP55: u8, const IP56: u8,
const IP57: u8, const IP58: u8, const IP59: u8, const IP60: u8,
const IP61: u8, const IP62: u8, const IP63: u8, const IP64: u8,
const EP01: u8, const EP02: u8, const EP03: u8, const EP04: u8,
const EP05: u8, const EP06: u8, const EP07: u8, const EP08: u8,
const EP09: u8, const EP10: u8, const EP11: u8, const EP12: u8,
const EP13: u8, const EP14: u8, const EP15: u8, const EP16: u8,
const EP17: u8, const EP18: u8, const EP19: u8, const EP20: u8,
const EP21: u8, const EP22: u8, const EP23: u8, const EP24: u8,
const EP25: u8, const EP26: u8, const EP27: u8, const EP28: u8,
const EP29: u8, const EP30: u8, const EP31: u8, const EP32: u8,
const EP33: u8, const EP34: u8, const EP35: u8, const EP36: u8,
const EP37: u8, const EP38: u8, const EP39: u8, const EP40: u8,
const EP41: u8, const EP42: u8, const EP43: u8, const EP44: u8,
const EP45: u8, const EP46: u8, const EP47: u8, const EP48: u8,
const TP01: u8, const TP02: u8, const TP03: u8, const TP04: u8,
const TP05: u8, const TP06: u8, const TP07: u8, const TP08: u8,
const TP09: u8, const TP10: u8, const TP11: u8, const TP12: u8,
const TP13: u8, const TP14: u8, const TP15: u8, const TP16: u8,
const TP17: u8, const TP18: u8, const TP19: u8, const TP20: u8,
const TP21: u8, const TP22: u8, const TP23: u8, const TP24: u8,
const TP25: u8, const TP26: u8, const TP27: u8, const TP28: u8,
const TP29: u8, const TP30: u8, const TP31: u8, const TP32: u8,
const S000: u8, const S001: u8, const S002: u8, const S003: u8,
const S004: u8, const S005: u8, const S006: u8, const S007: u8,
const S008: u8, const S009: u8, const S010: u8, const S011: u8,
const S012: u8, const S013: u8, const S014: u8, const S015: u8,
const S016: u8, const S017: u8, const S018: u8, const S019: u8,
const S020: u8, const S021: u8, const S022: u8, const S023: u8,
const S024: u8, const S025: u8, const S026: u8, const S027: u8,
const S028: u8, const S029: u8, const S030: u8, const S031: u8,
const S032: u8, const S033: u8, const S034: u8, const S035: u8,
const S036: u8, const S037: u8, const S038: u8, const S039: u8,
const S040: u8, const S041: u8, const S042: u8, const S043: u8,
const S044: u8, const S045: u8, const S046: u8, const S047: u8,
const S048: u8, const S049: u8, const S050: u8, const S051: u8,
const S052: u8, const S053: u8, const S054: u8, const S055: u8,
const S056: u8, const S057: u8, const S058: u8, const S059: u8,
const S060: u8, const S061: u8, const S062: u8, const S063: u8,
const S100: u8, const S101: u8, const S102: u8, const S103: u8,
const S104: u8, const S105: u8, const S106: u8, const S107: u8,
const S108: u8, const S109: u8, const S110: u8, const S111: u8,
const S112: u8, const S113: u8, const S114: u8, const S115: u8,
const S116: u8, const S117: u8, const S118: u8, const S119: u8,
const S120: u8, const S121: u8, const S122: u8, const S123: u8,
const S124: u8, const S125: u8, const S126: u8, const S127: u8,
const S128: u8, const S129: u8, const S130: u8, const S131: u8,
const S132: u8, const S133: u8, const S134: u8, const S135: u8,
const S136: u8, const S137: u8, const S138: u8, const S139: u8,
const S140: u8, const S141: u8, const S142: u8, const S143: u8,
const S144: u8, const S145: u8, const S146: u8, const S147: u8,
const S148: u8, const S149: u8, const S150: u8, const S151: u8,
const S152: u8, const S153: u8, const S154: u8, const S155: u8,
const S156: u8, const S157: u8, const S158: u8, const S159: u8,
const S160: u8, const S161: u8, const S162: u8, const S163: u8,
const S200: u8, const S201: u8, const S202: u8, const S203: u8,
const S204: u8, const S205: u8, const S206: u8, const S207: u8,
const S208: u8, const S209: u8, const S210: u8, const S211: u8,
const S212: u8, const S213: u8, const S214: u8, const S215: u8,
const S216: u8, const S217: u8, const S218: u8, const S219: u8,
const S220: u8, const S221: u8, const S222: u8, const S223: u8,
const S224: u8, const S225: u8, const S226: u8, const S227: u8,
const S228: u8, const S229: u8, const S230: u8, const S231: u8,
const S232: u8, const S233: u8, const S234: u8, const S235: u8,
const S236: u8, const S237: u8, const S238: u8, const S239: u8,
const S240: u8, const S241: u8, const S242: u8, const S243: u8,
const S244: u8, const S245: u8, const S246: u8, const S247: u8,
const S248: u8, const S249: u8, const S250: u8, const S251: u8,
const S252: u8, const S253: u8, const S254: u8, const S255: u8,
const S256: u8, const S257: u8, const S258: u8, const S259: u8,
const S260: u8, const S261: u8, const S262: u8, const S263: u8,
const S300: u8, const S301: u8, const S302: u8, const S303: u8,
const S304: u8, const S305: u8, const S306: u8, const S307: u8,
const S308: u8, const S309: u8, const S310: u8, const S311: u8,
const S312: u8, const S313: u8, const S314: u8, const S315: u8,
const S316: u8, const S317: u8, const S318: u8, const S319: u8,
const S320: u8, const S321: u8, const S322: u8, const S323: u8,
const S324: u8, const S325: u8, const S326: u8, const S327: u8,
const S328: u8, const S329: u8, const S330: u8, const S331: u8,
const S332: u8, const S333: u8, const S334: u8, const S335: u8,
const S336: u8, const S337: u8, const S338: u8, const S339: u8,
const S340: u8, const S341: u8, const S342: u8, const S343: u8,
const S344: u8, const S345: u8, const S346: u8, const S347: u8,
const S348: u8, const S349: u8, const S350: u8, const S351: u8,
const S352: u8, const S353: u8, const S354: u8, const S355: u8,
const S356: u8, const S357: u8, const S358: u8, const S359: u8,
const S360: u8, const S361: u8, const S362: u8, const S363: u8,
const S400: u8, const S401: u8, const S402: u8, const S403: u8,
const S404: u8, const S405: u8, const S406: u8, const S407: u8,
const S408: u8, const S409: u8, const S410: u8, const S411: u8,
const S412: u8, const S413: u8, const S414: u8, const S415: u8,
const S416: u8, const S417: u8, const S418: u8, const S419: u8,
const S420: u8, const S421: u8, const S422: u8, const S423: u8,
const S424: u8, const S425: u8, const S426: u8, const S427: u8,
const S428: u8, const S429: u8, const S430: u8, const S431: u8,
const S432: u8, const S433: u8, const S434: u8, const S435: u8,
const S436: u8, const S437: u8, const S438: u8, const S439: u8,
const S440: u8, const S441: u8, const S442: u8, const S443: u8,
const S444: u8, const S445: u8, const S446: u8, const S447: u8,
const S448: u8, const S449: u8, const S450: u8, const S451: u8,
const S452: u8, const S453: u8, const S454: u8, const S455: u8,
const S456: u8, const S457: u8, const S458: u8, const S459: u8,
const S460: u8, const S461: u8, const S462: u8, const S463: u8,
const S500: u8, const S501: u8, const S502: u8, const S503: u8,
const S504: u8, const S505: u8, const S506: u8, const S507: u8,
const S508: u8, const S509: u8, const S510: u8, const S511: u8,
const S512: u8, const S513: u8, const S514: u8, const S515: u8,
const S516: u8, const S517: u8, const S518: u8, const S519: u8,
const S520: u8, const S521: u8, const S522: u8, const S523: u8,
const S524: u8, const S525: u8, const S526: u8, const S527: u8,
const S528: u8, const S529: u8, const S530: u8, const S531: u8,
const S532: u8, const S533: u8, const S534: u8, const S535: u8,
const S536: u8, const S537: u8, const S538: u8, const S539: u8,
const S540: u8, const S541: u8, const S542: u8, const S543: u8,
const S544: u8, const S545: u8, const S546: u8, const S547: u8,
const S548: u8, const S549: u8, const S550: u8, const S551: u8,
const S552: u8, const S553: u8, const S554: u8, const S555: u8,
const S556: u8, const S557: u8, const S558: u8, const S559: u8,
const S560: u8, const S561: u8, const S562: u8, const S563: u8,
const S600: u8, const S601: u8, const S602: u8, const S603: u8,
const S604: u8, const S605: u8, const S606: u8, const S607: u8,
const S608: u8, const S609: u8, const S610: u8, const S611: u8,
const S612: u8, const S613: u8, const S614: u8, const S615: u8,
const S616: u8, const S617: u8, const S618: u8, const S619: u8,
const S620: u8, const S621: u8, const S622: u8, const S623: u8,
const S624: u8, const S625: u8, const S626: u8, const S627: u8,
const S628: u8, const S629: u8, const S630: u8, const S631: u8,
const S632: u8, const S633: u8, const S634: u8, const S635: u8,
const S636: u8, const S637: u8, const S638: u8, const S639: u8,
const S640: u8, const S641: u8, const S642: u8, const S643: u8,
const S644: u8, const S645: u8, const S646: u8, const S647: u8,
const S648: u8, const S649: u8, const S650: u8, const S651: u8,
const S652: u8, const S653: u8, const S654: u8, const S655: u8,
const S656: u8, const S657: u8, const S658: u8, const S659: u8,
const S660: u8, const S661: u8, const S662: u8, const S663: u8,
const S700: u8, const S701: u8, const S702: u8, const S703: u8,
const S704: u8, const S705: u8, const S706: u8, const S707: u8,
const S708: u8, const S709: u8, const S710: u8, const S711: u8,
const S712: u8, const S713: u8, const S714: u8, const S715: u8,
const S716: u8, const S717: u8, const S718: u8, const S719: u8,
const S720: u8, const S721: u8, const S722: u8, const S723: u8,
const S724: u8, const S725: u8, const S726: u8, const S727: u8,
const S728: u8, const S729: u8, const S730: u8, const S731: u8,
const S732: u8, const S733: u8, const S734: u8, const S735: u8,
const S736: u8, const S737: u8, const S738: u8, const S739: u8,
const S740: u8, const S741: u8, const S742: u8, const S743: u8,
const S744: u8, const S745: u8, const S746: u8, const S747: u8,
const S748: u8, const S749: u8, const S750: u8, const S751: u8,
const S752: u8, const S753: u8, const S754: u8, const S755: u8,
const S756: u8, const S757: u8, const S758: u8, const S759: u8,
const S760: u8, const S761: u8, const S762: u8, const S763: u8>
OFB<u64> for DES_Generic<ROUND, SHIFT,
PC101, PC102, PC103, PC104, PC105, PC106, PC107, PC108,
PC109, PC110, PC111, PC112, PC113, PC114, PC115, PC116,
PC117, PC118, PC119, PC120, PC121, PC122, PC123, PC124,
PC125, PC126, PC127, PC128, PC129, PC130, PC131, PC132,
PC133, PC134, PC135, PC136, PC137, PC138, PC139, PC140,
PC141, PC142, PC143, PC144, PC145, PC146, PC147, PC148,
PC149, PC150, PC151, PC152, PC153, PC154, PC155, PC156,
PC201, PC202, PC203, PC204, PC205, PC206, PC207, PC208,
PC209, PC210, PC211, PC212, PC213, PC214, PC215, PC216,
PC217, PC218, PC219, PC220, PC221, PC222, PC223, PC224,
PC225, PC226, PC227, PC228, PC229, PC230, PC231, PC232,
PC233, PC234, PC235, PC236, PC237, PC238, PC239, PC240,
PC241, PC242, PC243, PC244, PC245, PC246, PC247, PC248,
IP01, IP02, IP03, IP04, IP05, IP06, IP07, IP08,
IP09, IP10, IP11, IP12, IP13, IP14, IP15, IP16,
IP17, IP18, IP19, IP20, IP21, IP22, IP23, IP24,
IP25, IP26, IP27, IP28, IP29, IP30, IP31, IP32,
IP33, IP34, IP35, IP36, IP37, IP38, IP39, IP40,
IP41, IP42, IP43, IP44, IP45, IP46, IP47, IP48,
IP49, IP50, IP51, IP52, IP53, IP54, IP55, IP56,
IP57, IP58, IP59, IP60, IP61, IP62, IP63, IP64,
EP01, EP02, EP03, EP04, EP05, EP06, EP07, EP08,
EP09, EP10, EP11, EP12, EP13, EP14, EP15, EP16,
EP17, EP18, EP19, EP20, EP21, EP22, EP23, EP24,
EP25, EP26, EP27, EP28, EP29, EP30, EP31, EP32,
EP33, EP34, EP35, EP36, EP37, EP38, EP39, EP40,
EP41, EP42, EP43, EP44, EP45, EP46, EP47, EP48,
TP01, TP02, TP03, TP04, TP05, TP06, TP07, TP08,
TP09, TP10, TP11, TP12, TP13, TP14, TP15, TP16,
TP17, TP18, TP19, TP20, TP21, TP22, TP23, TP24,
TP25, TP26, TP27, TP28, TP29, TP30, TP31, TP32,
S000, S001, S002, S003, S004, S005, S006, S007,
S008, S009, S010, S011, S012, S013, S014, S015,
S016, S017, S018, S019, S020, S021, S022, S023,
S024, S025, S026, S027, S028, S029, S030, S031,
S032, S033, S034, S035, S036, S037, S038, S039,
S040, S041, S042, S043, S044, S045, S046, S047,
S048, S049, S050, S051, S052, S053, S054, S055,
S056, S057, S058, S059, S060, S061, S062, S063,
S100, S101, S102, S103, S104, S105, S106, S107,
S108, S109, S110, S111, S112, S113, S114, S115,
S116, S117, S118, S119, S120, S121, S122, S123,
S124, S125, S126, S127, S128, S129, S130, S131,
S132, S133, S134, S135, S136, S137, S138, S139,
S140, S141, S142, S143, S144, S145, S146, S147,
S148, S149, S150, S151, S152, S153, S154, S155,
S156, S157, S158, S159, S160, S161, S162, S163,
S200, S201, S202, S203, S204, S205, S206, S207,
S208, S209, S210, S211, S212, S213, S214, S215,
S216, S217, S218, S219, S220, S221, S222, S223,
S224, S225, S226, S227, S228, S229, S230, S231,
S232, S233, S234, S235, S236, S237, S238, S239,
S240, S241, S242, S243, S244, S245, S246, S247,
S248, S249, S250, S251, S252, S253, S254, S255,
S256, S257, S258, S259, S260, S261, S262, S263,
S300, S301, S302, S303, S304, S305, S306, S307,
S308, S309, S310, S311, S312, S313, S314, S315,
S316, S317, S318, S319, S320, S321, S322, S323,
S324, S325, S326, S327, S328, S329, S330, S331,
S332, S333, S334, S335, S336, S337, S338, S339,
S340, S341, S342, S343, S344, S345, S346, S347,
S348, S349, S350, S351, S352, S353, S354, S355,
S356, S357, S358, S359, S360, S361, S362, S363,
S400, S401, S402, S403, S404, S405, S406, S407,
S408, S409, S410, S411, S412, S413, S414, S415,
S416, S417, S418, S419, S420, S421, S422, S423,
S424, S425, S426, S427, S428, S429, S430, S431,
S432, S433, S434, S435, S436, S437, S438, S439,
S440, S441, S442, S443, S444, S445, S446, S447,
S448, S449, S450, S451, S452, S453, S454, S455,
S456, S457, S458, S459, S460, S461, S462, S463,
S500, S501, S502, S503, S504, S505, S506, S507,
S508, S509, S510, S511, S512, S513, S514, S515,
S516, S517, S518, S519, S520, S521, S522, S523,
S524, S525, S526, S527, S528, S529, S530, S531,
S532, S533, S534, S535, S536, S537, S538, S539,
S540, S541, S542, S543, S544, S545, S546, S547,
S548, S549, S550, S551, S552, S553, S554, S555,
S556, S557, S558, S559, S560, S561, S562, S563,
S600, S601, S602, S603, S604, S605, S606, S607,
S608, S609, S610, S611, S612, S613, S614, S615,
S616, S617, S618, S619, S620, S621, S622, S623,
S624, S625, S626, S627, S628, S629, S630, S631,
S632, S633, S634, S635, S636, S637, S638, S639,
S640, S641, S642, S643, S644, S645, S646, S647,
S648, S649, S650, S651, S652, S653, S654, S655,
S656, S657, S658, S659, S660, S661, S662, S663,
S700, S701, S702, S703, S704, S705, S706, S707,
S708, S709, S710, S711, S712, S713, S714, S715,
S716, S717, S718, S719, S720, S721, S722, S723,
S724, S725, S726, S727, S728, S729, S730, S731,
S732, S733, S734, S735, S736, S737, S738, S739,
S740, S741, S742, S743, S744, S745, S746, S747,
S748, S749, S750, S751, S752, S753, S754, S755,
S756, S757, S758, S759, S760, S761, S762, S763>
{
    fn encrypt(&mut self, mut iv: u64, from: *const u8, length_in_bytes: u64, to: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(from.add(progress as usize) as *const u64 ) };
            iv = self.encrypt_u64(iv);
            let coded = block ^ iv;
            unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), 8); }
            progress += 8;
        }

        if progress == length_in_bytes
        {
            self.set_success();
            progress
        }
        else
        {
            let mut block = 0_u64;
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { from.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let coded = block ^ self.encrypt_u64(iv);
            unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), tail); }
            self.set_success();
            progress + tail as u64
        }
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    fn encrypt_into_array<U, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
        {
            self.set_failed();
            return 0;
        }
        des_pre_encrypt_into_array!(cipher, length_in_bytes, U);
        self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    fn decrypt_into_array<U, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
        {
            self.set_failed();
            return 0;
        }
        des_pre_decrypt_into_array!(message, length_in_bytes, U);
        self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }
}