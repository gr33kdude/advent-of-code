#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::fmt;
use std::fs;
use std::process;

use std::collections::HashSet;
use itertools::Itertools;

fn usage(args: &Vec<String>) {
    println!("Usage: {} <input.txt>", args[0]);
    process::exit(1);
}

fn priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        (c as u8) - b'A' + 27
    } else if c.is_ascii_lowercase() {
        (c as u8) - b'a' + 1
    } else {
        0
    }
    .into()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let mut part1 = 0;

    let sacks = input.lines();
    for sack in sacks {
        let len = sack.chars().count();
        let (first_comp, second_comp) = sack.split_at(len/2);

        let first  = HashSet::<char>::from_iter(first_comp.chars());
        let second = HashSet::<char>::from_iter(second_comp.chars());

        for ch in first.intersection(&second) {
            part1 += priority(*ch);
        }
    }

    let mut part2 = 0;

    let sacks = input.lines();
    for mut chunk in &sacks.into_iter().chunks(3) {
        let first  = HashSet::<char>::from_iter(chunk.next().unwrap().chars());
        let second = HashSet::<char>::from_iter(chunk.next().unwrap().chars());
        let third  = HashSet::<char>::from_iter(chunk.next().unwrap().chars());

        let badge : HashSet<_> = first.intersection(&second)
            .cloned().collect();
        let badge = badge
            .intersection(&third);
        for ch in badge {
            part2 += priority(*ch);
        }
    }

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
