#![allow(warnings)]
#[macro_use] extern crate scan_fmt;

use std::env;
use std::fs;

#[derive(Debug)]
struct Config {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn part1(passwords: &[Config]) -> usize {
    passwords.iter().filter(|pass| {
        let count = pass.password
            .chars()
            .filter(|&c| c == pass.letter)
            .count();

        pass.min <= count && count <= pass.max
    }).count()
}

fn part2(passwords: &[Config]) -> usize {
    passwords.iter().filter(|pass| {
        (pass.password.as_bytes()[pass.min - 1] as char == pass.letter) !=
        (pass.password.as_bytes()[pass.max - 1] as char == pass.letter)
    }).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut passwords : Vec<Config> = contents
        .lines()
        .map(|s| scan_fmt!(s, "{d}-{d} {[a-z]}: {}", usize, usize, char, String)
             .map(|(a,b,c,d)| Config{min: a, max: b, letter: c, password: d}).expect("Fuck, bad input"))
        .collect();

    println!("Valid passwords: {}", part2(&passwords));
}