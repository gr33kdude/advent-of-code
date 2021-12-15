#![allow(warnings)]

use std::env;
use std::fs;
use std::io;

type int = i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let nums : Vec<int> = input
        .trim_end()
        .split(',')
        .map(|s| s.parse()
            .expect(
                &format!("not a number: {}", s)))
        .collect();

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    assert!( min <= max );

    // sweep from min to max, determine cost for all items by computing abs(diff)
    
    fn cost_1(i : int, position : int) -> int {
        (i - position).abs()
    }
    
    fn cost_2(i : int, position : int) -> int {
        let dist = (i - position).abs();
        (dist*(dist+1)) / 2
    }

    fn cost(nums : &[int], f : &dyn Fn(&int) -> int ) -> int {
        nums
            .iter()
            .map(f)
            .sum()
    }

    let cost_fns = [cost_1, cost_2];
    for (i, f) in cost_fns.iter().enumerate() {
        let costs = (min..=max)
            .map(|x| {
                nums
                    .iter()
                    .map(|p| f(x, *p))
                    .sum()
                });

        let minimum : int = costs
            .min()
            .unwrap();

        println!("Part {} : {}", i, minimum);
    }
}