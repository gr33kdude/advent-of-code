#![allow(warnings)]

use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() == 1 { "input.txt" } else { &args[1] };
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let groups = contents
        .trim_end()
        .split("\n\n");
    
    let mut total_count = 0;

    for (i, group) in groups.enumerate() {
        let ballots = group.split("\n").collect::<Vec<&str>>();

        let mut votes = HashMap::new();

        let n = ballots.len();
        for ballot in ballots {
            for question in ballot.chars() {
                let count = votes.entry(question).or_insert(0);
                *count += 1;
            }
        }

        for v in votes.values() {
            total_count += if *v == n { 1 } else { 0 }
        }
    }

    println!("{}", total_count);
}
