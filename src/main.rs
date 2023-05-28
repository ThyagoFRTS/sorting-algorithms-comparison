pub mod sort_methods;
pub mod config;
pub mod utils;
pub mod cmd;

use std::time::Instant;
use std::thread;
use crossbeam::scope;
use csv::{Writer, WriterBuilder};
use std::fs::OpenOptions;

use peak_alloc::PeakAlloc;

use crate::{config::Config, utils::{ execute_instance}, cmd::clear_console};

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
//"2023-06-23 17:50:03",quick,1000,melhor,tempo,memoria

fn main() {
    //vec![10000000, 1000000, 100000, 10000, 1000, 100],
    //let sizes = vec![100000, 10000, 1000, 100];
    let sizes = vec![10000000, 1000000];
    let methods = vec!["bubblesort", "selectionsort", "quicksort"];
    let cases = vec!["pior", "medio", "melhor"];
    

    for n in sizes {
        //clear_console();
        println!("Valor de N: {:?}",n);
        
        for case in cases.clone() {

            //for case in cases.clone() {
                scope(|s| {
                    //vec!["bubblesort", "selectionsort", "quicksort"];
                    s.spawn(|_| {
                        execute_instance(methods[0], n, case);
                    });
        
                    s.spawn(|_| {
                        execute_instance(methods[1], n, case);
                    });

                    s.spawn(|_| {
                        execute_instance(methods[2], n, case);
                    });
                })
                .unwrap();
        }
    }
}

