use rand::Rng;
use chrono::prelude::*;

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