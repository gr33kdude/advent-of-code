#![allow(warnings)]

use std::env;
use std::fs;
use std::io;

type int = i32;

const MAX : usize = 12;

type Summary = [int; MAX];
const EMPTY : Summary = [0; MAX];

fn to_summary(num : &int) -> Summary {
    let mut summary : Summary = EMPTY;
    for i in 0..summary.len() {
        let index = summary.len() - 1 - i;
        if num & (1 << index) == 0 {
            summary[i] = -1;
        } else {
            summary[i] = 1;
        }
    }

    summary
}

fn tuple_add(a : &Summary, b : &Summary) -> Summary {
    let mut summary : Summary = EMPTY;
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        summary[i] = x + y;
    }
    summary
}

fn summarize(nums : &[int]) -> Summary {
    nums
        .iter()
        .map(to_summary)
        .fold(EMPTY, |acc, x| tuple_add(&acc, &x))
}

// use a array of integers to represent if the bit should be 0 or 1
fn update(counts : &mut Summary, bits : int) -> () {
    //println!("BEGIN: {:?} -> {:05b}", counts, convert(counts));
    //println!("+ADD:  {:05b}", bits);

    for i in 0..counts.len() {
        let index = counts.len() - 1 - i;
        let bit = (bits & (1 << index)) > 0;
        let sign = if bit {1} else {-1};

        //println!("counts[{}] = {}, bit = {}", i, counts[i], bit);
        counts[i] += sign;
    }

    //println!("END:   {:?} -> {:05b}", counts, convert(counts));
}

fn convert(counts : &Summary) -> int {
    let mut bits = 0;
    for i in 0..counts.len() {
        let index = counts.len() - 1 - i;
        if counts[index] > 0 {
            bits |= 1 << i;
        }
    }

    bits
}

// determine how many 1s or 0s are in the sequence
// most:  
// least: if number is equal to nums, then it is still the least, otherwise it is the opposite

fn determine(nums : &[int], most : bool) -> int {
    let mut counts : [int; MAX] = [0; MAX];
    for &num in nums {
        update(&mut counts, num);
    }
    
    //println!("#: {} = {:?}", nums.len(), counts);

    for (i, count) in counts.iter_mut().enumerate() {
        if !most && (count.abs() < nums.len() as i32) {
            *count = -*count;
        }
    }
    convert(&counts)
}

fn find(nums : &[int], position : int, most : bool) -> int {
    match nums.len() {
        0 => return -1,
        1 => return nums[0],
        _ => {},
    }

    if position == MAX as i32 {
        panic!("Should have found a value by now.");
    }

    let mut counts : [int; MAX] = [0; MAX];
    let summary = summarize(nums);

    println!("SUMMARY: {:?}", summary);

    //summarize()
    //update(&counts, 

    /*
    nums
        .iter()
        .filter()
    */

    // examine numbers, filter for position
    // recurse with next position
    // finish if there's only 1 number in the list

    1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let nums : Vec<int> = input
        .lines()
        .map(|s| int::from_str_radix(s, 2).expect("parse error"))
        .collect();

    let  most : int = determine(&nums, true);
    let least : int = determine(&nums, false);

    println!("{} * {} = {}", most, least, most * least);
}