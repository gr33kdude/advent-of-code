#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::fmt;
use std::fs;
use std::ops;
use std::process;

use std::collections::HashMap;

fn usage(args: &Vec<String>) {
    println!("Usage: {} <input.txt>", args[0]);
    process::exit(1);
}

fn map_add(map: &mut HashMap<char, i32>, c: char) {
    map.entry(c).and_modify(|x| *x += 1).or_insert(1);
}

fn map_sub(map: &mut HashMap<char, i32>, c: char) {
    let e = map.remove_entry(&c);
    if let Some( (k, v) ) = e {
        if v > 1 {
            map.insert(k, v-1);
        }
    }
}

fn sequence_start(s: &str) -> usize {
    let mut map = HashMap::new();

    const WINDOW : usize = 14;

    for c in s.chars().take(WINDOW) {
        map_add(&mut map, c);
    }

    let mut head = s.chars().skip(WINDOW);
    for (i, old_c) in s.chars().enumerate() {
        if let Some(c) = head.next() {
            map_sub(&mut map, old_c);
            map_add(&mut map, c);

            if map.keys().count() == WINDOW {
                return i+WINDOW+1;
            }
        } else {
            return i+WINDOW;
        }
    }
    unreachable!("Did not expect to exhaust tail iterator");
}

fn run(filename: &str) -> usize {
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");
    let input = input.trim();

    sequence_start(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }
/*
    let filename = &args[1];
    println!("{}", run(filename));
*/    
    test_part2();
}

fn test_part1() {
    assert_eq!(run("example1.txt"), 7);
    assert_eq!(run("example2.txt"), 5);
    assert_eq!(run("example3.txt"), 6);
    assert_eq!(run("example4.txt"), 10);
    assert_eq!(run("example5.txt"), 11);
}

fn test_part2() {
    assert_eq!(run("example1.txt"), 19);
    assert_eq!(run("example2.txt"), 23);
    assert_eq!(run("example3.txt"), 23);
    assert_eq!(run("example4.txt"), 29);
    assert_eq!(run("example5.txt"), 26);
}