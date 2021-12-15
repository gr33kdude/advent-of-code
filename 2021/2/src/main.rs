#![allow(warnings)]

use std::env;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Command {
    direction : Direction,
    magnitude : i32,
}

enum Direction2 {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn to_cmd(s: &str) -> Command {
    let dir_word_letter = s.chars().nth(0);
    let mag_s = s.split(' ').nth(1).expect("invalid command");
    let mag = mag_s.parse::<i32>().expect("number parse error");

    match dir_word_letter {
        Some('f') => Command { direction: Direction::Forward, magnitude: mag },
        Some('u') => Command { direction: Direction::Up, magnitude: mag },
        Some('d') => Command { direction: Direction::Down, magnitude: mag },
        _ => unreachable!(),
    }
}

fn commands(lines : &str) -> Vec<Command> {
    lines
        .lines()
        .map(to_cmd)
        .collect()
}

fn process(lines : &str, dir : Direction) -> i32 {
    lines
        .lines()
        .map(|s| { let cmd = to_cmd(s); if cmd.direction == dir { cmd.magnitude } else { 0 } })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("Failed to read file");

    let x    = process(&input, Direction::Forward);
    let up   = process(&input, Direction::Up);
    let down = process(&input, Direction::Down);

    let part1 = x * (down - up);

    let cmds = commands(&input);
    let mut aim = 0;
    let mut depth = 0;
    for cmd in cmds {
        match cmd.direction {
            Direction::Up => aim -= cmd.magnitude,
            Direction::Down => aim += cmd.magnitude,
            Direction::Forward => depth += aim * cmd.magnitude,
        }
    }
    let part2 = x * depth;

    println!("{}\n{}", part1, part2);
}
