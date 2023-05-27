pub mod sort_methods;
pub mod config;
pub mod utils;
pub mod cmd;

use std::time::Instant;
use std::io::Write;
use std::fs::File;
use csv::{Writer, WriterBuilder};
use std::fs::OpenOptions;

use peak_alloc::PeakAlloc;

use crate::{config::Config, sort_methods::execute, utils::{get_current_date, get_vector_from}, cmd::clear_console};

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
//"2023-06-23 17:50:03",quick,1000,melhor,tempo,memoria

fn main() {

    let conf = Config::new();
    
    let file_path = "output.csv";

    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open the file");

    
    let mut csv_writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    
    
    for n in conf.sizes().into_iter() {
        clear_console();
        println!("Valor de N: {:?}",n);
        for case in conf.cases().into_iter() {
            let mut memory = 0;
            let mut time = 0;
            println!("Caso: {:?}",case);

            for method in conf.methods().into_iter() {
                println!("Algoritmo de N: {:?}",method);
                if case == "medio" {
                    for i in 0..5 {
                        println!("Iteracao: {:?}",i);
                        let mut arr = get_vector_from(case, n.clone());

                        let start_time = Instant::now();
                        execute(&method, &mut arr);
                        let end_time = Instant::now();
    
                        let execution_time = end_time.duration_since(start_time);
                        time += execution_time.as_secs();
                        memory += PEAK_ALLOC.peak_usage_as_mb() as i32;
                    }

                    time = time / 5;
                    memory = memory / 5;

                    let line = vec![get_current_date(),method,n.to_string(), case.to_string(), time.to_string(), memory.to_string()];
                    
                    csv_writer.write_record(&line).expect("Failed to write record to CSV");
                    csv_writer.flush().expect("Failed to flush CSV writer");
                    
                } else {
                    let mut arr = get_vector_from(case, n.clone());

                    let start_time = Instant::now();
                    execute(&method, &mut arr);
                    let end_time = Instant::now();

                    let execution_time = end_time.duration_since(start_time);
                    time += execution_time.as_secs();
                    memory += PEAK_ALLOC.peak_usage_as_mb() as i32;

                    let line = vec![get_current_date(),method,n.to_string(), case.to_string(), time.to_string(), memory.to_string()];
                    csv_writer.write_record(&line).expect("Failed to write record to CSV");
                    csv_writer.flush().expect("Failed to flush CSV writer");
                }
            }

        }

    }
    
    
}

