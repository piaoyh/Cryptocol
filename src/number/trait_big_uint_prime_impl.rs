// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a big unsigned integer struct
//! with user-defined fixed size and its methods.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use crate::number::{ SmallUInt, TraitsBigUInt, BigUInt, BigUInt_Prime, BigUInt_Modular };
use crate::number::biguint_calc_to_calc_assign;


impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    pub(super) fn common_gcd_uint<U>(&self, other: U) -> Self
    where U: TraitsBigUInt<U>
    {
        let mut y = self.wrapping_rem_uint(other);
        let mut x = other;
        while !y.is_zero()
        {
            let t = y;
            y = x.wrapping_rem(y);
            x = t;
        }
        Self::from_uint(x)
    }

    pub(super) fn common_gcd(&self, other: &Self) -> Self
    {
        let mut x = self.clone();
        let mut y = Self::from_biguint(other);
        let mut t: Self;
        while !y.is_zero()
        {
            t = y;
            y = x.wrapping_rem(&t);
            x = t;
        }
        x
    }

    // Filter out composite numbers.
    // If self is filtered out because it is a composite number,
    // it returns true. Otherwise, it returns false
    pub(crate) fn filter_out_composite_number(&self) -> bool
    {
        if self.is_zero_or_one() || self.is_even()
            { return true; }

        let a_list = [3_u8, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        let len = a_list.len();
        for i in 0..len
        {
            if self.wrapping_rem_uint(a_list[i]).is_zero()
                { return true; }
        }
        false
    }
    
    /// Performs Millar Rabin method with a number less than `self`.
    pub(crate) fn test_miller_rabin(&self, a: &Self) -> bool
    {
        let self_minus_one = self.wrapping_sub_uint(1_u8);
        let mut d = self_minus_one.clone();
        let mut s = 0_u64;
        while d.is_even()
        {
            d.shift_right_assign(1_u8);
            s += 1;
        }
        let mut x = a.modular_pow(&d, self);
        if x == self_minus_one || x.is_one()
            { return true; }
        for _ in 0..s-1
        {
            x.modular_pow_assign_uint(2_u8, self);
            if x.is_one()
                { return false; }
            if x == self_minus_one
                { return true; }
        }
        false
    }
}


impl<T, const N: usize> BigUInt_Prime<T, N> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{    
    /*** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS ***/

    fn gcd_uint<U>(&self, other: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }
        self.common_gcd_uint(other)
    }

    fn gcd_assign_uint<U>(&mut self, other: U)
    where U: TraitsBigUInt<U>
    {
        biguint_calc_to_calc_assign!(self, Self::gcd_uint, other);
    }

    fn lcm_uint<U>(&self, other: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if self.is_zero() || other.is_zero()
           { panic!(); }
        self.wrapping_div(&self.gcd_uint(other)).wrapping_mul_uint(other)
    }

    fn lcm_assign_uint<U>(&mut self, other: U)
    where U: TraitsBigUInt<U>
    {
        biguint_calc_to_calc_assign!(self, Self::lcm_uint, other);
    }

    fn gcd(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }
        self.common_gcd(other)
    }

    fn gcd_assign(&mut self, other: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::gcd, other);
    }

    // fn modular_gcd(&self, other: &Self, modulus: &Self) -> Self
    // {
    //     let me = self.wrapping_rem(modulus);
    //     let it = other.wrapping_rem(modulus);
    //     if me.is_zero() || it.is_zero()
    //         { panic!(); }
    //     common_gcd(&me, &it)
    // }

    // fn modular_gcd_assign(&mut self, other: &Self, modulus: &Self)
    // {
    //     biguint_calc_to_calc_assign!(self, Self::modular_gcd, other, modulus);
    // }

    fn extended_gcd(&self, other: &Self) -> (Self, Self, Self)
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }

        let mut a = self.clone();
        let mut b = BigUInt::<T, N>::from_biguint(other);
        let mut x0 = BigUInt::<T, N>::one();
        let mut y0 = BigUInt::<T, N>::zero();
        let mut x1 = BigUInt::<T, N>::zero();
        let mut y1 = BigUInt::<T, N>::one();
        let mut t: BigUInt<T, N>;
        let mut q: BigUInt<T, N>;
        let mut x0_flags = 0_u8;
        let mut y0_flags = 0_u8;
        while !b.is_zero()
        {
            q = a.wrapping_div(&b);

            t = x1;
            x1 = x0.wrapping_sub(&q.wrapping_mul(&t));
            x0 = t;
            x0_flags |= x0.get_all_flags();

            t = y1;
            y1 = y0.wrapping_sub(&q.wrapping_mul(&t));
            y0 = t;
            y0_flags |= y0.get_all_flags();
            
            t = b;
            b = a.wrapping_rem(&t);
            a = t;
        }
        x0.set_all_flags(x0_flags);
        y0.set_all_flags(y0_flags);
        (a, x0, y0)
    }

    // fn modular_extended_gcd(&self, other: &Self, modulus:  &Self) -> (Self, Self, Self)
    // {
    //     let mut a = self.wrapping_rem(modulus);
    //     let mut b = other.wrapping_rem(modulus);
    //     if a.is_zero() || b.is_zero()
    //         { panic!(); }

    //     let mut x0 = BigUInt::<T, N>::one();
    //     let mut y0 = BigUInt::<T, N>::zero();
    //     let mut x1 = BigUInt::<T, N>::zero();
    //     let mut y1 = BigUInt::<T, N>::one();
    //     let mut t: BigUInt<T, N>;
    //     let mut q: BigUInt<T, N>;
    //     let mut x0_flags = 0_u8;
    //     let mut y0_flags = 0_u8;
    //     while !b.is_zero()
    //     {
    //         q = a.modular_div(&b, modulus);

    //         t = x1.clone();
    //         x1 = x0.modular_sub(&q.modular_mul(&x1, modulus), modulus);
    //         x0 = t;
    //         x0_flags |= x0.get_all_flags();

    //         t = y1.clone();
    //         y1 = y0.modular_sub(&q.modular_mul(&y1, modulus), modulus);
    //         y0 = t;
    //         y0_flags |= y0.get_all_flags();
            
    //         t = b;
    //         b = a.modular_rem(&t, modulus);
    //         a = t;
    //     }
    //     x0.set_all_flags(x0_flags);
    //     y0.set_all_flags(y0_flags);
    //     (a, x0, y0)
    // }
    
    fn lcm(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
           { panic!(); }
        self.wrapping_div(&self.gcd(&other)).wrapping_mul(&other)
    }

    fn lcm_assign(&mut self, other: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::lcm, other);
    }


    /*** METHODS FOR PRIME NUMBER TEST ***/

    fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool
    {
        if self.filter_out_composite_number()
            { return false; }
        
        if self.le_uint(u128::MAX)
        {
            let small_self = self.into_u128();
            return small_self.is_prime_using_miller_rabin(repetition);
        }

        let a_list = [73_u16, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173];
        let len = a_list.len();
        let common = if len < repetition {len} else {repetition};
        for i in 0..common
        {
            if !self.test_miller_rabin(&Self::from_uint(a_list[i]))
                { return false; }
        }

        let mut a = a_list[len-1] as u32 + 2;
        for _ in common..repetition
        {
            if !self.test_miller_rabin(&Self::from_uint(a))
                { return false; }
            a += 2;
        }
        true
    }
}

/*
fn collect_small_prime_numbers(n: usize)
{
    let mut primes = vec![2_u32, 3];
    let mut i = 3;
    while primes.len() < n
    {
        let mut is_prime = true;
        for k in primes.clone()
        {
            if i % k == 0
            {
                is_prime = false;
                break;
            }
        }
        if is_prime
            { primes.push(i); }
        i += 2;
    }

    print!("Primes: [");
    for prime in primes
        { print!("{prime}, "); }
    println!("]");
}
    
impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    pub fn is_prime_using_miller_rabin_simultaneously(&self, repetition: usize) -> bool
    {
        use std::thread::available_parallelism;
        if self.is_zero_or_one() || self.is_even()
            { return false; }
        
        if self.le_uint(u128::MAX)
        {
            let small_self = self.into_u128();
            return small_self.is_prime_using_miller_rabin(repetition);
        }

        let a_list = [2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        let len = a_list.len();
        for i in 1..len
        {
            if self.wrapping_rem_uint(a_list[i]).is_zero()
                { return false; }
        }

        let number_of_threads: usize = match available_parallelism()
        {
            Ok(non_zero) => non_zero.get() as usize,
            Err(_) => 1_usize,
        };
        
        if number_of_threads == 1
        {
            self.is_prime_using_miller_rabin_sequentially(repetition, a_list)
        }
        else
        {
            use std::sync::{Mutex, Arc};
            use std::sync::mpsc::channel;
            use std::thread::spawn;
            use crate::concurrency::do_simultaneously_unit;

            let mut b_list= Vec::new();
            let common = if len < repetition {len} else {repetition};
            for i in 0..common
                { b_list.push(a_list[i] as u32); }
            let mut a = a_list[len-1] as u32 + 2;
            for _ in common..repetition
            {
                b_list.push(a);
                a += 2;
            }

            // let me = self.clone();
            // let mut jobs = Vec::new();
            // for i in 0..repetition
            // {
            //     // let me = self.clone();
            //     // let a = b_list[i];
            //     jobs.push(|me: Self, a: u32| { me.test_miller_rabin(&Self::from_uint(a)) });
            // }

            let mut threads = Vec::new();
            let (tx, rx) = channel::<(Arc<Mutex<bool>>, Box<Self>, u32)>();
            let receiver = Arc::new(Mutex::new(rx));
            for _ in 0..number_of_threads
            {
                let rxx = receiver.clone();
                threads.push(spawn(move ||
                {
                    loop
                    {
                        let r = rxx.lock().unwrap();
                        match r.recv()
                        {
                            Ok((res, me, a)) => {
                                drop(r);
                                *res.lock().unwrap() &= (*me).test_miller_rabin(&Self::from_uint(a));
                            },
                            _ => { return; },
                        }
                    }
                }));
            }

            let ark_mut_res = Arc::new(Mutex::new(true));
            for i in 0..repetition
            {
                tx.clone().send((ark_mut_res.clone(), Box::new(self.clone()), b_list[i])).unwrap();
            }
            drop(tx);
            for thread in threads
                { thread.join().unwrap(); }
            *ark_mut_res.lock().unwrap()
        }
    }
}
*/
