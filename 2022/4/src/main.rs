#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::fmt;
use std::fs;
use std::ops;
use std::process;

extern crate interval;
extern crate gcollections;

use crate::interval::Interval;
use crate::interval::ops::*;
use gcollections::ops::*;

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

    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let mut assignments = line.split(",");
        let first = assignments.nth(0).unwrap();
        let mut first = first.split("-");
        let first = Interval::new(
            first.nth(0).unwrap().parse::<i32>().unwrap(),
            first.nth(0).unwrap().parse::<i32>().unwrap()
        );
        
        let second = assignments.nth(0).unwrap();
        let mut second = second.split("-");
        let second = Interval::new(
            second.nth(0).unwrap().parse::<i32>().unwrap(),
            second.nth(0).unwrap().parse::<i32>().unwrap()
        );

        if first.is_subset(&second) || second.is_subset(&first) {
            part1 += 1;
        }

        if first.overlap(&second) {
            part2 += 1;
        }
    }

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
