#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::fmt;
use std::fs;
use std::process;

fn usage(args: &Vec<String>) {
    println!("Usage: {} <input.txt>", args[0]);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let part1 = 0;
    let part2 = 0;

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
