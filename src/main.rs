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

use crate::{config::Config, sort_methods::execute, utils::{get_current_date, get_vector_from, write_on_csv, execute_instance}, cmd::clear_console};

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
//"2023-06-23 17:50:03",quick,1000,melhor,tempo,memoria

fn main() {

    let sizes = vec![100000, 10000, 1000, 100];
    let methods = vec!["bubblesort", "selectionsort", "quicksort"];
    let cases = vec!["pior", "medio", "melhor"];
    
    let file_path = "output.csv";

    /*
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open the file");

    
    let mut csv_writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
     */
    
    
    for n in sizes {
        //clear_console();
        println!("Valor de N: {:?}",n);
        
        for method in methods.clone() {

            for case in cases.clone() {
                
                
                let thread_handle = thread::spawn(move || {
                    

                    //for case in ["pior", "medio", "melhor"] {

                       execute_instance(method, n, case);
                    //}
                    /*
                    let mut arr = get_vector_from(case, n);
                    
                    println!("Alg: {:?} n: {:?} case: {:?} status: started", method, n, case);
                    let start_time = Instant::now();
                    execute(&method, &mut arr);
                    let time = start_time.elapsed().as_secs();
                    println!("Alg: {:?} n: {:?} case: {:?} status: finished", method, n, case);
                    
                    let memory_usage = PEAK_ALLOC.peak_usage_as_mb() as i32;;
                    
                    let line = vec![get_current_date(),method.to_string(),n.to_string(), case.to_string(), time.to_string(), memory_usage.to_string()];
                    
                    write_on_csv(line, csv_writer);
                    */
                });

                thread_handle.join().expect("Erro ao aguardar a thread");
                
            }
            
        }
        

    }
    
    
}

