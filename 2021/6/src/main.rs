#![allow(warnings)]

use std::{env,fs,io};
use std::time::{Duration,Instant};

type int = usize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let mut nums : Vec<int>  = input
        .trim_end()
        .split(',')
        .map(|x| x.parse::<int>().expect(&format!("not a number: {}", x)))
        .collect();

    println!("Day 0: {:?}", nums);

    let mut fish : [int; 9] = [0; 9];
    for &num in &nums {
        fish[num as usize] += 1;
    }

    for day in 1..=256 {
        let new = fish[0];
        for i in 0..fish.len()-1 {
            fish[i] = fish[i+1];
        }
        fish[6] += new;
        fish[8] = new;
    }


    println!("{}", fish.iter().sum::<int>())
}
