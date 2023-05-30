use std::fs::File;
use csv::{Writer, WriterBuilder};
use std::time::Instant;
use rand::Rng;
use chrono::prelude::*;
use std::fs::OpenOptions;

use peak_alloc::PeakAlloc;

use crate::{sort_methods::execute, PEAK_ALLOC};



pub fn ascending_vector(n: usize) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::with_capacity(n);
    for i in 0..n {
        vector.push(i as i32);
    }
    vector
}

pub fn descending_vector(n: usize) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::with_capacity(n);
    for i in (0..n).rev() {
        vector.push(i as i32);
    }
    vector
}

pub fn random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vector: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        vector.push(rng.gen_range(0..n.try_into().unwrap()));
    }
    vector
}

pub fn get_current_date() -> String {
    let current_datetime = Local::now();
    current_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
//cases: vec!["pior".to_string(), "medio".to_string(), "melhor".to_string()],
pub fn get_vector_from(case: &str, n: usize) -> Vec<i32> {
    match case{
        "pior" => return descending_vector(n),
        "medio" => return random_vector(n),
        "melhor" => return ascending_vector(n),
        _ => return vec![],
    }
}

pub fn write_on_csv(line: Vec<String>, mut csv_writer: Writer<File>) {      
    csv_writer.write_record(&line).expect("Failed to write record to CSV");
    csv_writer.flush().expect("Failed to flush CSV writer");
}

pub fn execute_instance(method: &str,n: usize,case: &str){
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("output.csv")
        .expect("Failed to open the file");

                        
    let csv_writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    let date = get_current_date();

    let mut arr = get_vector_from(case, n);

    println!("[{:?}] alg: {:?} n: {:?} case: {:?} status: started", date, method, n, case);
    let start_time = Instant::now();
    execute(&method, &mut arr);
    let time = start_time.elapsed().as_secs();

    let final_date = get_current_date();
    println!("[{:?}] alg: {:?} n: {:?} case: {:?} status: finished", final_date, method, n, case);

    let memory_usage = PEAK_ALLOC.peak_usage_as_mb() as i32;
    let line = vec![date,method.to_string(),n.to_string(), case.to_string(), time.to_string(), memory_usage.to_string()];

    write_on_csv(line, csv_writer);
}