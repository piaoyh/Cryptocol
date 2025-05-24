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
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, BigUInt, BigUInt_Modular };
use crate::number::biguint_calc_assign_to_calc;
use crate::number::trait_big_uint_more_impl::common_next_multiple_of_assign;



macro_rules! biguint_modular_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        if $modulo.is_zero_or_one()
            { panic!(); }
        $func($me, $rhs, $modulo);
    }
    // biguint_modular_calc_assign!(self, common_modular_add_assign_uint, rhs, modulo);
    //
    // if modulo.is_zero_or_one()
    //     { panic!(); }
    // self.common_modular_add_assign_uint(rhs, modulo);
}
pub(super) use biguint_modular_calc_assign;

macro_rules! biguint_general_modular_calc_pow_assign
{
    ($me:expr, $one:expr, $exp:expr, $modulo:expr) => {
        let mut res = BigUInt::<T, N>::one();
        let mut mexp = $exp.clone();
        while !mexp.is_zero()
        {
            if mexp.is_odd()
                { res.modular_mul_assign($me, $modulo); }
            $me.modular_mul_assign(&$me.clone(), $modulo);
            mexp >>= $one;
        }
        $me.set_number(res.get_number());
        $me.set_flag_bit(res.get_all_flags());
    }
    // biguint_general_modular_calc_pow_assign!(self, 1, exp, modulo);
    //
    // let mut res = Self::one();
    // let mut mexp = exp.clone();
    // while !mexp.is_zero()
    // {
    //     if mexp.is_odd()
    //         { res.modular_mul_assign(self, modulo); }
    //     self.modular_mul_assign(&self.clone(), modulo);
    //     mexp >>= 1;
    // }
    // self.set_number(res.get_number());
    // self.set_flag_bit(res.get_all_flags());
}
pub(super) use biguint_general_modular_calc_pow_assign;


pub(super) fn common_modular_add_assign_uint<T, U, const N: usize>(me: &mut BigUInt<T, N>, rhs: U, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    if *me >= *modulo
    {
        me.wrapping_rem_assign(modulo);
        me.reset_all_flags();
    }

    if rhs.length_in_bytes() > T::size_in_bytes()  // && (module <= rhs)
    {
        common_modular_add_assign(me, &BigUInt::<T, N>::from_uint(rhs), modulo);
    }
    else if modulo.gt_uint(rhs)
    {
        if !rhs.is_zero()
        {
            let diff = modulo.wrapping_sub_uint(rhs);
            if *me >= diff
            {
                me.wrapping_sub_assign(&diff);
                flags |= BigUInt::<T, N>::OVERFLOW;
            }
            else    // if *me < diff
            {
                me.wrapping_add_assign_uint(rhs);
            }
        }
    }
    else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (me < module <= rhs)
    {
        let modu = modulo.into_uint::<U>();
        let mrhs = rhs.wrapping_rem(modu);
        if !mrhs.is_zero()
        {
            let mut mself = me.into_uint::<U>();
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
            me.set_uint(mself);
        }
    }
    me.set_all_flags(flags);
}

pub(super) fn common_modular_add_assign<T, const N: usize>(me: &mut BigUInt<T, N>, rhs: &BigUInt<T, N>, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    if *me >= *modulo
        { me.wrapping_rem_assign(modulo); }

    let _rhs: BigUInt<T, N>;
    let mrhs: &BigUInt<T, N>;
    if *rhs >= *modulo
    {
        _rhs = rhs.wrapping_rem(modulo);
        mrhs = &_rhs;
    }
    else
    {
        mrhs = rhs;
    }
    let diff = modulo.wrapping_sub(mrhs);
    if *me >= diff
    {
        me.wrapping_sub_assign(&diff);
        flags |= BigUInt::<T, N>::OVERFLOW;
    }
    else
    {
        me.wrapping_add_assign(mrhs);
    }
    me.set_all_flags(flags);
}

pub(super) fn common_modular_sub_assign_uint<T, U, const N: usize>(me: &mut BigUInt<T, N>, rhs: U, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    if *me >= *modulo
    {
        me.wrapping_rem_assign(modulo);
        me.reset_all_flags();
    }

    if rhs.length_in_bytes() > T::size_in_bytes()  // if rhs.length_in_bytes() > T::size_in_bytes() && (module <= rhs)
    {
        common_modular_sub_assign(me, &BigUInt::<T, N>::from_uint(rhs), modulo);
    }
    else if modulo.gt_uint(rhs)
    {
        if !rhs.is_zero()
        {
            if me.ge_uint(rhs)    // if *me >= rhs
            {
                me.wrapping_sub_assign_uint(rhs);
            }
            else    // if *me < rhs
            {
                let diff = modulo.wrapping_sub_uint(rhs);
                me.wrapping_add_assign(&diff);
                flags |= BigUInt::<T, N>::UNDERFLOW;
            }
        }
    }
    else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (me < module <= rhs)
    {
        let modu = modulo.into_uint::<U>();
        let mrhs = rhs.wrapping_rem(modu);
        if !mrhs.is_zero()
        {
            let mself = me.into_uint::<U>().modular_sub(mrhs, modu);
            if mself >= mrhs
            {
                me.set_uint(mself.wrapping_sub(mrhs));
            }
            else    // if mself < rhs
            {
                let diff = modu.wrapping_sub(mrhs);
                me.set_uint(mself.wrapping_add(diff));
                flags |= BigUInt::<T, N>::UNDERFLOW;
            }
        }
    }
    me.set_flag_bit(flags);
}

pub(super) fn common_modular_sub_assign<T, const N: usize>(me: &mut BigUInt<T, N>, rhs: &BigUInt<T, N>, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    me.wrapping_rem_assign(modulo);
    if *rhs < *modulo    // In this case, it does not cause cloning for performance.
    {
        // let mrhs = rhs.clone(); // Does not clone for performance
        if *me >= *rhs
        {
            me.wrapping_sub_assign(rhs);
        }
        else    // if *me < *rhs
        {
            let diff = modulo.wrapping_sub(rhs);
            me.wrapping_add_assign(&diff);
            flags |= BigUInt::<T, N>::UNDERFLOW;
        }
    }
    else    // if *rhs >= *modulo
    {
        let mrhs = rhs.wrapping_rem(modulo);
        if *me >= mrhs
        {
            me.wrapping_sub_assign(&mrhs);
        }
        else
        {
            let diff = modulo.wrapping_sub(&mrhs);
            me.wrapping_add_assign(&diff);
            flags |= BigUInt::<T, N>::UNDERFLOW;
        }
    }
    me.set_all_flags(flags);
}

pub(super) fn common_modular_mul_assign_uint<T, U, const N: usize>(me: &mut BigUInt<T, N>, rhs: U, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    if *me >= *modulo
    {
        me.wrapping_rem_assign(modulo);
        me.reset_all_flags();
    }

    if !me.is_zero()
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            common_modular_mul_assign(me, &BigUInt::<T, N>::from_uint(rhs), modulo);
        }
        else if modulo.gt_uint(rhs)
        {
            let mut mrhs = T::num::<U>(rhs);
            let mut res = BigUInt::<T, N>::zero();
            while mrhs > T::zero()
            {
                if mrhs.is_odd()
                    { res.modular_add_assign(me, modulo); }
                me.modular_add_assign(&me.clone(), modulo);
                mrhs >>= T::one();
            }
            *me = res;
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (self < module <= rhs)
        {
            let modu = modulo.into_uint::<U>();
            let mrhs = rhs.wrapping_rem(modu);
            if mrhs.is_zero()
            {
                me.set_zero();
            }
            else
            {
                let mself = me.into_uint::<U>();
                let new_mself = mself.modular_mul(mrhs, modu);
                me.set_uint(new_mself);
                if mself > new_mself
                {
                    flags |= BigUInt::<T, N>::OVERFLOW;
                }
                else
                {
                    let r = mself.overflowing_mul(mrhs);
                    if (r.0 >= modu) || r.1
                        { flags |= BigUInt::<T, N>::OVERFLOW; }
                }
            }
        }
    }
    me.set_flag_bit(flags);
}

pub(super) fn common_modular_mul_assign<T, const N: usize>(me: &mut BigUInt<T, N>, rhs: &BigUInt<T, N>, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut flags = me.get_all_flags();
    if *me >= *modulo
        { me.wrapping_rem_assign(modulo); }

    if !me.is_zero()
    {
        let mut mrhs = rhs.wrapping_rem(modulo);
        let mut res = BigUInt::<T, N>::zero();
        let zero = BigUInt::<T, N>::zero();
        while mrhs > zero
        {
            if mrhs.is_odd()
                { res.modular_add_assign(me, modulo); }
            me.modular_add_assign(&me.clone(), modulo);
            mrhs.shift_right_assign(1_u8);
        }
        me.set_number(res.get_number());
        if res.is_overflow()
            { flags |= BigUInt::<T, N>::OVERFLOW; }
    }
    me.set_all_flags(flags);
}

pub(super) fn common_modular_pow_assign_uint<T, U, const N: usize>(me: &mut BigUInt<T, N>, exp: U, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    biguint_general_modular_calc_pow_assign!(me, U::u8_as_smalluint(1), exp, modulo);
}

pub(super) fn common_modular_pow_assign<T, const N: usize>(me: &mut BigUInt<T, N>, exp: &BigUInt<T, N>, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    biguint_general_modular_calc_pow_assign!(me, 1, exp, modulo);
}

pub(super) fn common_modular_next_multiple_of_assign_uint<T, U, const N: usize>(me: &mut BigUInt<T, N>, rhs: U, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    if *me >= *modulo
        { me.wrapping_rem_assign(modulo); }

    if U::size_in_bytes() > T::size_in_bytes()
    {
        common_next_multiple_of_assign(me, &BigUInt::from_uint(rhs));
    }
    else if modulo.gt_uint(rhs)
    {
        let diff = me.wrapping_rem_uint(rhs);
        if !diff.is_zero()
            { me.modular_add_assign_uint(rhs - diff, modulo); }
    }
    else    // if U::size_in_bytes() <= T::size_in_bytes() and modulo <= rhs
    {
        let trhs = T::num(rhs);
        let diff = me.wrapping_rem_uint(trhs);
        if !diff.is_zero()
            { me.modular_add_assign_uint(trhs - diff, modulo); }
    }
}

pub(super) fn common_modular_next_multiple_of_assign<T, const N: usize>(me: &mut BigUInt<T, N>, rhs: &BigUInt<T, N>, modulo: &BigUInt<T, N>)
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    me.wrapping_rem_assign(modulo);
    let mrhs;
    if rhs.ge(modulo)
        { mrhs = rhs.wrapping_rem(modulo); }
    else
        { mrhs = rhs.clone(); }
    let r = me.wrapping_rem(&mrhs);
    if !r.is_zero()
        { me.modular_add_assign(&mrhs.wrapping_sub(&r), modulo); }
}



impl<T, const N: usize> BigUInt_Modular<T, N> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /*** ADDITION UINT ***/

    fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        biguint_calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulo);
    }

    fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_modular_calc_assign!(self, common_modular_add_assign_uint, rhs, modulo);
    }
    
    fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_add_assign, rhs, modulo);
    }

    fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        biguint_modular_calc_assign!(self, common_modular_add_assign, rhs, modulo);
    }



    /*** SUBTRACTION ***/

    fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::modular_sub_assign_uint, rhs, modulo);
    }

    fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_modular_calc_assign!(self, common_modular_sub_assign_uint, rhs, modulo);
    }

    fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_sub_assign, rhs, modulo);
    }

    fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        biguint_modular_calc_assign!(self, common_modular_sub_assign, rhs, modulo);
    }

    

    /*** MULTIPLICATION ***/

    fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::modular_mul_assign_uint, rhs, modulo);
    }

    fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_modular_calc_assign!(self, common_modular_mul_assign_uint, rhs, modulo);
    }

    fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_mul_assign, rhs, modulo);
    }

    fn modular_mul_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        biguint_modular_calc_assign!(self, common_modular_mul_assign, rhs, modulo);
    }



    /*** DIVISION ***/

    fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::modular_div_assign_uint, rhs, modulo);
    }

    fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero_or_one() | rhs.is_zero()
            { panic!(); }

        let mut mrhs = rhs;
        if modulo.le_uint(rhs)
        {
            let modu = modulo.into_uint::<U>();
            mrhs = rhs.wrapping_rem(modu);
            if mrhs.is_zero()
                { panic!(); }
        }

        let flags = self.get_all_flags();   
        if *self >= *modulo
        {
            self.wrapping_rem_assign(modulo);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            self.modular_div_assign(&Self::from_uint(mrhs), modulo);
        }
        else if modulo.gt_uint(rhs)
        {
            self.wrapping_div_assign_uint(mrhs);
        }
        else
        {
            let modu = modulo.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_div(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_div_assign, rhs, modulo);
    }

    fn modular_div_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        self.wrapping_div_assign(&rhs.wrapping_rem(modulo));
    }

    fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_rem_assign_uint(rhs, modulo);
        res.into_uint::<U>()
    }

    fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero_or_one() | rhs.is_zero()
            { panic!(); }

        let mut mrhs = rhs;
        if modulo.le_uint(rhs)
        {
            let modu = modulo.into_uint::<U>();
            mrhs = rhs.wrapping_rem(modu);
            if mrhs.is_zero()
                { panic!(); }
        }

        let flags = self.get_all_flags();   
        if *self >= *modulo
        {
            self.wrapping_rem_assign(modulo);
            self.reset_all_flags();
        }

        if rhs.length_in_bytes() > T::size_in_bytes()  // if rhs.length_in_bytes() > T::size_in_bytes() && (module <= rhs)
        {
            self.modular_rem_assign(&Self::from_uint(rhs), modulo);
        }
        else if modulo.gt_uint(rhs)
        {
            self.wrapping_rem_assign_uint(rhs);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (modulo <= rhs)
        {
            let modu = modulo.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_rem(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_rem_assign, rhs, modulo);
    }

    fn modular_rem_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        self.wrapping_rem_assign(&rhs.wrapping_rem(modulo));
    }



    /*** METHODS FOR EXPONENTIATION AND LOGARITHM ***/

    fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::modular_pow_assign_uint, exp, modulo);
    }

    fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero_or_one() || (self.is_zero() && exp.is_zero())
            { panic!(); }
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mexp =
            if modulo.le_uint(exp)
                { exp.wrapping_rem(modulo.into_uint::<U>()) }
            else
                { exp };
        if self.is_zero() && mexp.is_zero()
            { panic!(); }
        common_modular_pow_assign_uint(self, mexp, modulo);
    }

    fn modular_pow(&self, exp: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_pow_assign, exp, modulo);
    }

    fn modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
    {
        if modulo.is_zero_or_one() || (self.is_zero() && exp.is_zero())
            { panic!(); }
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mexp =
            if *modulo <= *exp
                { exp.wrapping_rem(modulo) }
            else
                { exp.clone() };
        if self.is_zero() && mexp.is_zero()
            { panic!(); }
        
        common_modular_pow_assign(self, &mexp, modulo);
    }



    /*** MULTIPLE ***/

    fn modular_next_multiple_of_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::modular_next_multiple_of_assign_uint, rhs, modulo);
    }

    fn modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_modular_calc_assign!(self, common_modular_next_multiple_of_assign_uint, rhs, modulo);
    }

    fn modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::modular_next_multiple_of_assign, rhs, modulo);
    }

    fn modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        biguint_modular_calc_assign!(self, common_modular_next_multiple_of_assign, rhs, modulo);
    }
}
