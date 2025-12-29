// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ Display, Debug };
use std::str::FromStr;
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, TraitsBigUInt, BigUInt, BigUInt_Modular };
use crate::number::biguint_calc_assign_to_calc;



macro_rules! biguint_modular_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr, $modulus:expr) => {
        if $modulus.is_zero_or_one()
            { panic!(); }
        $func($me, $rhs, $modulus);
    }
    // biguint_modular_calc_assign!(self, common_modular_add_assign_uint, rhs, modulus);
    //
    // if modulus.is_zero_or_one()
    //     { panic!(); }
    // self.common_modular_add_assign_uint(rhs, modulus);
}

macro_rules! biguint_general_modular_calc_pow_assign
{
    ($me:expr, $one:expr, $exp:expr, $modulus:expr) => {
        let mut res = BigUInt::<T, N>::one();
        let mut mexp = $exp.clone();
        while !mexp.is_zero()
        {
            if mexp.is_odd()
                { res.modular_mul_assign($me, $modulus); }
            $me.modular_mul_assign(&$me.clone(), $modulus);
            mexp >>= $one;
        }
        $me.set_number(res.get_number());
        $me.set_flag_bit(res.get_all_flags());
    }
    // biguint_general_modular_calc_pow_assign!(self, 1, exp, modulus);
    //
    // let mut res = Self::one();
    // let mut mexp = exp.clone();
    // while !mexp.is_zero()
    // {
    //     if mexp.is_odd()
    //         { res.modular_mul_assign(self, modulus); }
    //     self.modular_mul_assign(&self.clone(), modulus);
    //     mexp >>= 1;
    // }
    // self.set_number(res.get_number());
    // self.set_flag_bit(res.get_all_flags());
}



impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>,
    Self: Sized + Clone + Display + Debug + ToString
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitXor<Self, Output = Self> + BitXorAssign
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
    {
    pub(super) fn common_modular_add_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        let mut flags = self.get_all_flags();
        if *self >= *modulus
        {
            self.wrapping_rem_assign(modulus);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()  // && (module <= rhs)
        {
            self.common_modular_add_assign(&Self::from_uint(rhs), modulus);
        }
        else if modulus.gt_uint(rhs)
        {
            if !rhs.is_zero()
            {
                let diff = modulus.wrapping_sub_uint(rhs);
                if *self >= diff
                {
                    self.wrapping_sub_assign(&diff);
                    flags |= Self::OVERFLOW;
                }
                else    // if *me < diff
                {
                    self.wrapping_add_assign_uint(rhs);
                }
            }
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (me < module <= rhs)
        {
            let modu = modulus.into_uint::<U>();
            let mrhs = rhs.wrapping_rem(modu);
            if !mrhs.is_zero()
            {
                let mut mself = self.into_uint::<U>();
                let diff = modu.wrapping_sub(mrhs);
                if mself >= diff
                {
                    mself = mself.wrapping_sub(diff);
                    flags |= BigUInt::<T, N>::OVERFLOW;
                }
                else    // if mself < diff
                {
                    mself = mself.wrapping_add(rhs);
                }
                self.set_uint(mself);
            }
        }
        self.set_all_flags(flags);
    }

    pub(super) fn common_modular_add_assign(&mut self, rhs: &Self, modulus: &Self)
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        let mut flags = self.get_all_flags();
        if *self >= *modulus
            { self.wrapping_rem_assign(modulus); }

        let _rhs: Self;
        let mrhs: &Self;
        if *rhs >= *modulus
        {
            _rhs = rhs.wrapping_rem(modulus);
            mrhs = &_rhs;
        }
        else
        {
            mrhs = rhs;
        }
        let diff = modulus.wrapping_sub(mrhs);
        if *self >= diff
        {
            self.wrapping_sub_assign(&diff);
            flags |= Self::OVERFLOW;
        }
        else
        {
            self.wrapping_add_assign(mrhs);
        }
        self.set_all_flags(flags);
    }

    pub(super) fn common_modular_sub_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        let mut flags = self.get_all_flags();
        if *self >= *modulus
        {
            self.wrapping_rem_assign(modulus);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()  // if rhs.length_in_bytes() > T::size_in_bytes() && (module <= rhs)
        {
            self.common_modular_sub_assign(&Self::from_uint(rhs), modulus);
        }
        else if modulus.gt_uint(rhs)
        {
            if !rhs.is_zero()
            {
                if self.ge_uint(rhs)    // if *me >= rhs
                {
                    self.wrapping_sub_assign_uint(rhs);
                }
                else    // if *me < rhs
                {
                    let diff = modulus.wrapping_sub_uint(rhs);
                    self.wrapping_add_assign(&diff);
                    flags |= Self::UNDERFLOW;
                }
            }
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (me < module <= rhs)
        {
            let modu = modulus.into_uint::<U>();
            let mrhs = rhs.wrapping_rem(modu);
            if !mrhs.is_zero()
            {
                let mself = self.into_uint::<U>().modular_sub(mrhs, modu);
                if mself >= mrhs
                {
                    self.set_uint(mself.wrapping_sub(mrhs));
                }
                else    // if mself < rhs
                {
                    let diff = modu.wrapping_sub(mrhs);
                    self.set_uint(mself.wrapping_add(diff));
                    flags |= Self::UNDERFLOW;
                }
            }
        }
        self.set_flag_bit(flags);
    }

    pub(super) fn common_modular_sub_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        let mut flags = self.get_all_flags();
        self.wrapping_rem_assign(modulus);
        if *rhs < *modulus    // In this case, it does not cause cloning for performance.
        {
            // let mrhs = rhs.clone(); // Does not clone for performance
            if *self >= *rhs
            {
                self.wrapping_sub_assign(rhs);
            }
            else    // if *me < *rhs
            {
                let diff = modulus.wrapping_sub(rhs);
                self.wrapping_add_assign(&diff);
                flags |= Self::UNDERFLOW;
            }
        }
        else    // if *rhs >= *modulus
        {
            let mrhs = rhs.wrapping_rem(modulus);
            if *self >= mrhs
            {
                self.wrapping_sub_assign(&mrhs);
            }
            else
            {
                let diff = modulus.wrapping_sub(&mrhs);
                self.wrapping_add_assign(&diff);
                flags |= Self::UNDERFLOW;
            }
        }
        self.set_all_flags(flags);
    }

    pub(super) fn common_modular_mul_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        let mut flags = self.get_all_flags();
        if *self >= *modulus
        {
            self.wrapping_rem_assign(modulus);
            self.reset_all_flags();
        }

        if !self.is_zero()
        {
            if U::size_in_bytes() > T::size_in_bytes()
            {
                self.common_modular_mul_assign(&Self::from_uint(rhs), modulus);
            }
            else if modulus.gt_uint(rhs)
            {
                let mut mrhs = T::num::<U>(rhs);
                let mut res = Self::zero();
                while mrhs > T::zero()
                {
                    if mrhs.is_odd()
                        { res.modular_add_assign(self, modulus); }
                    self.modular_add_assign(&self.clone(), modulus);
                    mrhs >>= T::one();
                }
                *self = res;
            }
            else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (self < module <= rhs)
            {
                let modu = modulus.into_uint::<U>();
                let mrhs = rhs.wrapping_rem(modu);
                if mrhs.is_zero()
                {
                    self.set_zero();
                }
                else
                {
                    let mself = self.into_uint::<U>();
                    let new_mself = mself.modular_mul(mrhs, modu);
                    self.set_uint(new_mself);
                    if mself > new_mself
                    {
                        flags |= Self::OVERFLOW;
                    }
                    else
                    {
                        let r = mself.overflowing_mul(mrhs);
                        if (r.0 >= modu) || r.1
                            { flags |= Self::OVERFLOW; }
                    }
                }
            }
        }
        self.set_flag_bit(flags);
    }

    pub(super) fn common_modular_mul_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        let mut flags = self.get_all_flags();
        if *self >= *modulus
            { self.wrapping_rem_assign(modulus); }

        if !self.is_zero()
        {
            let mut mrhs = rhs.wrapping_rem(modulus);
            let mut res = Self::zero();
            let zero = Self::zero();
            while mrhs > zero
            {
                if mrhs.is_odd()
                    { res.modular_add_assign(self, modulus); }
                self.modular_add_assign(&self.clone(), modulus);
                mrhs.shift_right_assign(1_u8);
            }
            self.set_number(res.get_number());
            if res.is_overflow()
                { flags |= Self::OVERFLOW; }
        }
        self.set_all_flags(flags);
    }

    pub(super) fn common_modular_pow_assign_uint<U>(&mut self, exp: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        biguint_general_modular_calc_pow_assign!(self, U::u8_as_smalluint(1), exp, modulus);
    }

    pub(super) fn common_modular_pow_assign(&mut self, exp: &Self, modulus: &Self)
    {
        biguint_general_modular_calc_pow_assign!(self, 1, exp, modulus);
    }

    pub(super) fn common_modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        if *self >= *modulus
            { self.wrapping_rem_assign(modulus); }

        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.common_next_multiple_of_assign(&Self::from_uint(rhs));
        }
        else if modulus.gt_uint(rhs)
        {
            let diff = self.wrapping_rem_uint(rhs);
            if !diff.is_zero()
                { self.modular_add_assign_uint(rhs - diff, modulus); }
        }
        else    // if U::size_in_bytes() <= T::size_in_bytes() and modulus <= rhs
        {
            let trhs = T::num(rhs);
            let diff = self.wrapping_rem_uint(trhs);
            if !diff.is_zero()
                { self.modular_add_assign_uint(trhs - diff, modulus); }
        }
    }

    pub(super) fn common_modular_next_multiple_of_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        self.wrapping_rem_assign(modulus);
        let mrhs;
        if rhs.ge(modulus)
            { mrhs = rhs.wrapping_rem(modulus); }
        else
            { mrhs = rhs.clone(); }
        let r = self.wrapping_rem(&mrhs);
        if !r.is_zero()
            { self.modular_add_assign(&mrhs.wrapping_sub(&r), modulus); }
    }
}


impl<T, const N: usize> BigUInt_Modular<T, N> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    /*** ADDITION UINT ***/

    fn modular_add_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>

    {
        biguint_calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulus);
    }

    fn modular_add_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        biguint_modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulus);
    }
    
    fn modular_add(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_add_assign, rhs, modulus);
    }

    fn modular_add_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        biguint_modular_calc_assign!(self, Self::common_modular_add_assign, rhs, modulus);
    }



    /*** SUBTRACTION ***/

    fn modular_sub_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::modular_sub_assign_uint, rhs, modulus);
    }

    fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        biguint_modular_calc_assign!(self, Self::common_modular_sub_assign_uint, rhs, modulus);
    }

    fn modular_sub(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_sub_assign, rhs, modulus);
    }

    fn modular_sub_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        biguint_modular_calc_assign!(self, Self::common_modular_sub_assign, rhs, modulus);
    }

    

    /*** MULTIPLICATION ***/

    fn modular_mul_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::modular_mul_assign_uint, rhs, modulus);
    }

    fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        biguint_modular_calc_assign!(self, Self::common_modular_mul_assign_uint, rhs, modulus);
    }

    fn modular_mul(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_mul_assign, rhs, modulus);
    }

    fn modular_mul_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        biguint_modular_calc_assign!(self, Self::common_modular_mul_assign, rhs, modulus);
    }



    /*** DIVISION ***/

    fn modular_div_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::modular_div_assign_uint, rhs, modulus);
    }

    fn modular_div_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        if modulus.is_zero_or_one() | rhs.is_zero()
            { panic!(); }

        let mut mrhs = rhs;
        if modulus.le_uint(rhs)
        {
            let modu = modulus.into_uint::<U>();
            mrhs = rhs.wrapping_rem(modu);
            if mrhs.is_zero()
                { panic!(); }
        }

        let flags = self.get_all_flags();   
        if *self >= *modulus
        {
            self.wrapping_rem_assign(modulus);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            self.modular_div_assign(&Self::from_uint(mrhs), modulus);
        }
        else if modulus.gt_uint(rhs)
        {
            self.wrapping_div_assign_uint(mrhs);
        }
        else
        {
            let modu = modulus.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_div(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn modular_div(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_div_assign, rhs, modulus);
    }

    fn modular_div_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        self.wrapping_rem_assign(modulus);
        self.wrapping_div_assign(&rhs.wrapping_rem(modulus));
    }

    fn modular_rem_uint<U>(&self, rhs: U, modulus: &Self) -> U
    where U: TraitsBigUInt<U>
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_rem_assign_uint(rhs, modulus);
        res.into_uint::<U>()
    }

    fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        if modulus.is_zero_or_one() | rhs.is_zero()
            { panic!(); }

        let mut mrhs = rhs;
        if modulus.le_uint(rhs)
        {
            let modu = modulus.into_uint::<U>();
            mrhs = rhs.wrapping_rem(modu);
            if mrhs.is_zero()
                { panic!(); }
        }

        let flags = self.get_all_flags();   
        if *self >= *modulus
        {
            self.wrapping_rem_assign(modulus);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()  // if rhs.length_in_bytes() > T::size_in_bytes() && (module <= rhs)
        {
            self.modular_rem_assign(&Self::from_uint(rhs), modulus);
        }
        else if modulus.gt_uint(rhs)
        {
            self.wrapping_rem_assign_uint(rhs);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (modulus <= rhs)
        {
            let modu = modulus.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_rem(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn modular_rem(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_rem_assign, rhs, modulus);
    }

    fn modular_rem_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        self.wrapping_rem_assign(modulus);
        self.wrapping_rem_assign(&rhs.wrapping_rem(modulus));
    }



    /*** METHODS FOR EXPONENTIATION AND LOGARITHM ***/

    fn modular_pow_uint<U>(&self, exp: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::modular_pow_assign_uint, exp, modulus);
    }

    fn modular_pow_assign_uint<U>(&mut self, exp: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        if modulus.is_zero_or_one() || (self.is_zero() && exp.is_zero())
            { panic!(); }
        if *self >= *modulus
            { self.wrapping_rem_assign(modulus); }
        let mexp =
            if modulus.le_uint(exp)
                { exp.wrapping_rem(modulus.into_uint::<U>()) }
            else
                { exp };
        if self.is_zero() && mexp.is_zero()
            { panic!(); }
        self.common_modular_pow_assign_uint(mexp, modulus);
    }

    fn modular_pow(&self, exp: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_pow_assign, exp, modulus);
    }

    fn modular_pow_assign(&mut self, exp: &Self, modulus: &Self)
    {
        if modulus.is_zero_or_one() || (self.is_zero() && exp.is_zero())
            { panic!(); }
        if *self >= *modulus
            { self.wrapping_rem_assign(modulus); }
        let mexp =
            if *modulus <= *exp
                { exp.wrapping_rem(modulus) }
            else
                { exp.clone() };
        if self.is_zero() && mexp.is_zero()
            { panic!(); }
        
        self.common_modular_pow_assign(&mexp, modulus);
    }



    /*** MULTIPLE ***/

    fn modular_next_multiple_of_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::modular_next_multiple_of_assign_uint, rhs, modulus);
    }

    fn modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: TraitsBigUInt<U>
    {
        biguint_modular_calc_assign!(self, Self::common_modular_next_multiple_of_assign_uint, rhs, modulus);
    }

    fn modular_next_multiple_of(&self, rhs: &Self, modulus: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_next_multiple_of_assign, rhs, modulus);
    }

    fn modular_next_multiple_of_assign(&mut self, rhs: &Self, modulus: &Self)
    {
        biguint_modular_calc_assign!(self, Self::common_modular_next_multiple_of_assign, rhs, modulus);
    }
}
