#![allow(warnings)]

use std::env;
use std::fs;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

fn find_tuple(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut first_idx = 0;
    let mut second_idx = nums.len() - 1;

    while first_idx < second_idx {
        let first = nums[first_idx];
        let second = nums[second_idx];

        //println!("Considering ({}, {}, {}): ", first, second, third);

        if first + second < target {
            first_idx += 1;
        } else if first + second == target {
            return Some((first, second))
        } else {
            second_idx -= 1;
        }
    }

    None
}

fn find_triple(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let mut first_idx = 0;
    let mut second_idx = 1;
    let mut third_idx = nums.len() - 1;

    while first_idx < second_idx && second_idx < third_idx {
        let first = nums[first_idx];
        let second = nums[second_idx];
        let third = nums[third_idx];

        //println!("Considering ({}, {}, {}): ", first, second, third);

        if first + second + third < target {
            if first_idx + 1 == second_idx {
                second_idx += 1;
            } else {
                let next_first = nums[first_idx + 1];
                let next_second = nums[second_idx + 1];

                if next_first - first < next_second - second {
                    first_idx += 1;
                } else {
                    second_idx += 1;
                }
            }
        } else if first + second + third == target {
            return Some((first, second, third))
        } else {
            third_idx -= 1;
        }
    }

    None
}

fn find_triple_hash(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let mut set = HashSet::new();

    for num in nums {
        set.insert(num);
    }

    for (i, x) in nums.iter().cloned().enumerate() {
        for (j, y) in nums.iter().cloned().enumerate() {
            if i == j {
                continue;
            }

            if set.contains(&(target - x - y)) {
                return Some((x, y, target - x - y));
            }
        }
    }

    None
}

fn test_find_triple(vec: &[i32], target: i32) {
    let mut rng = rand::thread_rng();
    let mut total = 100;
    let mut passed = 0;
    let mut failed = 0;

    for i in 0..total {
        let test_vals: Vec<i32> = vec
            .choose_multiple(&mut rng, 3)
            .cloned()
            .collect();
        let test_first = test_vals[0];
        let test_second = test_vals[1];
        let test_third = test_vals[2];

        let test_triple: (i32, i32, i32) = (test_first, test_second, test_third);
        let test_sum = test_first + test_second + test_third;

        match find_triple_hash(vec, test_sum) {
            Some(found_triple) => {
                let (first, second, third) = found_triple;
                let found_sum = first + second + third;

                if found_sum == test_sum {
                    passed += 1;
                    //println!("PASSED: {:?}", found_triple);
                } else {
                    failed += 1;
                    //println!("FAILED: Expected: {:?}; Found: {:?}",
                    //         test_triple, found_triple);
                }
            },
            _ => {
                failed += 1;
                //println!("FAILED: Expected to find {:?} but did not.", test_triple);
            },
        }
    }

    println!("TOTAL: {}, PASSED: {}, FAILED: {}", total, passed, failed);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let mut nums : Vec<i32> = input
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    nums.sort();

    let target = 2020;

    match find_triple_hash(&nums, target) {
        Some((first, second, third)) => {
            println!("{} + {} + {} == {} !!", first, second, third, target);
            println!("Answer: {}", first * second * third);
        },
        _ => (),
    }
}