#![allow(warnings)]

use std::fs;
use std::env;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    
    let input = fs::read_to_string(filename)
        .expect("Failed to read file"); 

    let depths : Vec<i32> = input
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let part1_count = depths.iter()
        .tuple_windows()
        .filter(|&(prev, cur)| cur > prev)
        .count();

    let part2_count = depths.iter()
        .tuple_windows()
        .tuple_windows()
        .filter(|&((a, _, _), (_, _, f))| f > a)
        .count();

    let D = depths.iter();
    let part2_count_alt = D.clone().zip(D.skip(3));

    println!("{}", part1_count);
    println!("{}", part2_count);
}
