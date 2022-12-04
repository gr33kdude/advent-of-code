#![allow(dead_code)]

use std::env;
use std::fmt;
use std::fs;
use std::process;

#[derive(fmt::Debug, PartialEq, Copy, Clone)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

fn deser_weapon(c : char) -> Weapon {
    match c {
        'A' | 'X' => Weapon::Rock,
        'B' | 'Y' => Weapon::Paper,
        'C' | 'Z' => Weapon::Scissors,
        _         => panic!("you fool!")
    }
}

// returns the weapon type that would win against the input weapon
fn winning(weapon : Weapon) -> Weapon {
    match weapon {
        Weapon::Rock => Weapon::Paper,
        Weapon::Paper => Weapon::Scissors,
        Weapon::Scissors => Weapon::Rock,
    }
}

#[derive(fmt::Debug, PartialEq, Copy, Clone)]
enum Result {
    Loss,
    Draw,
    Win,
}

fn deser_outcome(c : char) -> Result {
    match c {
        'X' => Result::Loss,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _   => panic!("you fool!"),
    }
}

fn compete(player : Weapon, opponent : Weapon) -> Result {
    if player == opponent {
        Result::Draw
    } else if player == winning(opponent) {
        Result::Win
    } else {
        Result::Loss
    }
}

fn match_fixing(opponent : Weapon, outcome : Result) -> Weapon {
    match outcome {
        Result::Draw => opponent,
        Result::Win => winning(opponent),
        Result::Loss => winning(winning(opponent)),
    }
}

fn score(player : Weapon, opponent : Weapon) -> i32 {
    let result = compete(player, opponent);
    
    3 * (result as i32) + (player as i32) + 1
}

fn usage(args : &Vec<String>) {
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
    let input = input
        .trim_end_matches("\n");

    let rounds = input.split("\n");

    let mut part1 = 0;
    let mut part2 = 0;
    for round in rounds {
        let mut round_it = round.chars();

        let opponent_c = round_it.nth(0).unwrap();
        let mine_c     = round_it.nth(1).unwrap();

        let opponent = deser_weapon(opponent_c);
        let mine     = deser_weapon(mine_c);
        let outcome  = deser_outcome(mine_c);

        let my_fixed = match_fixing(opponent, outcome);

        part1 += score(mine, opponent);
        part2 += score(my_fixed, opponent);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
