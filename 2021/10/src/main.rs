#![allow(warnings)]

use std::{env,fs,io};

type int = i32;

#[derive(Debug)]
enum Status {
    Complete,
    Incomplete(char),
    Corrupt(char),
}

fn verify(line: &str) -> Status {
    let count = 0;
    for c in line.chars() {
        
        println!("{}", c);
    }

    Status::Complete
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let lines: Vec<&str> = input
        .lines()
        .collect();

    for line in lines {
        println!("{:?}", verify(line));
    }
}