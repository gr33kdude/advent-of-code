#![allow(warnings)]

use std::env;
use std::fs;
use std::process;

fn usage(args : &Vec<String>) {
    println!("Usage: {} <filename>", args[0]);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) {
        usage(&args);
    }

    let filename = &args[1];
    
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");
    let input = input
        .strip_suffix("\n")
        .unwrap_or(""); 

    let elves = input.split("\n\n");
    let num_elves = elves.clone().count();

    let calories : Vec<usize> = elves
        .map(|s| s.split("\n")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .sum())
        .collect();

    let mut sorted_calories = calories
        .clone();
    sorted_calories.sort_unstable_by(|a, b| b.cmp(a));

    let max_calorie_elf = sorted_calories[0];

    let part1 = max_calorie_elf;
    let part2 : usize = sorted_calories.iter().take(3).sum();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
