#![allow(warnings)]
#[macro_use] extern crate scan_fmt;

use std::env;
use std::fs;

type Slope = (usize, usize);

fn trees(forest: Vec<&str>, slopes: &[Slope]) -> Vec<usize> {
    let mut treelist: Vec<usize> = Vec::new();
    for slope in slopes.iter() {
        let (mut row, mut col) = (0, 0);
        let (rows, cols) = (forest.len(), forest[0].len());

        let mut trees = 0;
        let mut done = false;
        while !done {
            if row >= rows {
                done = true;
                continue;
            }

            if forest[row].chars().nth(col) == Some('#') {
                trees += 1;
            }

            row += slope.1;
            col = (col + slope.0) % cols;
        }

        treelist.push(trees);
    }

    treelist
}


fn part1(forest: Vec<&str>) -> Vec<usize> {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    trees(forest, &slopes)
}

fn part2(forest: Vec<&str>) -> Vec<usize> {
    trees(forest, &[ (3, 1) ])
}

fn prod(arr: Vec<usize>) -> usize {
    arr.iter().fold(1, |acc, x| acc * x)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() == 1 { "input.txt" } else { &args[1] };
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let forest : Vec<&str> = contents
        .lines()
        .collect();

    println!("Part 1: {}", prod(part1(forest)));
    println!("Part 2: {}", prod(part2(forest)));
}
