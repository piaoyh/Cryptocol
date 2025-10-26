// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.



use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;
use std::thread::{ spawn, available_parallelism };

/// Utility function for concurrent operation that return unit type.
/// 
#[allow(dead_code)]
pub fn do_simultaneously_unit(jobs: Vec<fn()>)
{
    let number_of_threads: usize = match available_parallelism()
    {
        Ok(non_zero) => non_zero.get() as usize,
        Err(_) => 1_usize,
    };
    
    if number_of_threads == 1
    {
        for work in jobs
            { work(); }
        return;
    }

    let mut threads = Vec::new();
    let (tx, rx) = channel::<fn()>();
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
                    Ok(work) => { drop(r); work(); },
                    _ => { return },
                }
            }
        }));
    }

    for job in jobs
        { tx.clone().send(job).unwrap(); }
    drop(tx);
    for thread in threads
        { thread.join().unwrap(); }
}

// pub fn do_simultaneously_bool(thread: VecDeque<fn()>) -> bool
// {
//     let number_of_threads: usize;
//     match available_parallelism()
//     {
//         Ok(non_zero) => { number_of_threads = non_zero.get(); },
//         Err(_) => { number_of_threads = 1; },
//     }
    
//     if number_of_threads == 1
//     {
//         for work in thread
//             { work(); }
//         return;
//     }

//     let mutex = Arc::new(Mutex::new(thread));
//     let mut handles = Vec::new();

//     for _ in 0..number_of_threads
//     {
//         let m = mutex.clone();
//         let handle = spawn(move ||
//             {
//                 loop
//                 {
//                     let work: fn();
//                     {
//                         match m.lock()
//                         {
//                             Ok(mut jobs) => {
//                                     match jobs.pop_front()
//                                     {
//                                         Some(j) => work = j,
//                                         _ => return,
//                                     }
//                                 },
//                             _ => { return; },
//                         }
//                     }
//                     work();
//                 }
//             }
//         );
//         handles.push(handle);
//     }

//     for handle in handles
//         { handle.join().unwrap(); }
// }
