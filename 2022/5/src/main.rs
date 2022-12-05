#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::fmt;
use std::fs;
use std::ops;
use std::process;

fn usage(args: &Vec<String>) {
    println!("Usage: {} <input.txt>", args[0]);
    process::exit(1);
}

const MAX : usize = 9;

fn print_top(stacks : &[Vec<char>; MAX]) {
    for s in stacks {
        let top = s.len();
        if top == 0 {
            continue;
        } else {
            print!("{}", s[top-1]);
        }
    }
    println!();
}

fn print_stack(stacks: &[Vec<char>; MAX]) {
    for (i, s) in stacks.iter().enumerate() {
        print!("{}: ", i+1);
        for c in s {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let stacks : [&str; MAX] = [
        "TZB",
        "NDTHV",
        "DMFB",
        "LQVWGJT",
        "MQFVPGDW",
        "SFHGQZV",
        "WCTLRNSZ",
        "MRNJDWHZ",
        "SDFLQM",
    ];
    /*
    let stacks : [&str; MAX] = [
        "NZ",
        "DCM",
        "P",
    ];
    */    
    let mut stacks : [Vec<char>; MAX] = stacks
        .map(|s| s.chars().rev().collect());

    for line in input.lines() {
        if ! line.starts_with("move") {
            continue
        }
        
        println!("L: {}", line);

        print_stack(&stacks);

        let v : Vec<&str> = line.split(" ").collect();
        let qty = v[1].parse::<u8>().unwrap() as usize;

        //let qty = ((line.chars().nth(5).unwrap() as u8) - b'0') as usize;
        let src = (v[3].parse::<u8>().unwrap() - 1) as usize;
        let dst = (v[5].parse::<u8>().unwrap() - 1) as usize;

        println!("qty: {}, len: {}", qty, stacks[src].len());

        let at = stacks[src].len() - qty;
        let mut vals = stacks[src].split_off(at);//.unwrap();
        stacks[dst].append(&mut vals);

        println!("move {:?} from {:?} to {:?}", qty, src+1, dst+1);
        print_top(&stacks);
    }

    print_top(&stacks);
}