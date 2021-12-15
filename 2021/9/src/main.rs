#![allow(warnings)]

use std::env;
use std::fs;
use std::io;
use std::cmp;

type int = i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let lines = input.lines();
    let rows = lines.clone().count();

    let mut matrix : Vec<Vec<int>> = Vec::new();
    for line in lines {
        matrix.push( line
            .chars()
            .map(|x| x.to_digit(10).expect("not a number") as i32)
            .collect()
        )
    }

    let rows : int = matrix.len().try_into().unwrap();
    let cols : int = matrix[0].len().try_into().unwrap();

    let bound = |i, max| [cmp::max(0, i-1), cmp::min(max, i+1)];

    fn fuck_rust(matrix : &Vec<Vec<int>>, x : int, y : int) -> int {
        matrix[x as usize][y as usize] // TILTEDDDDDD xD
    }

    println!("rows = {}, cols = {}", rows, cols);
    let mut mins : Vec<int> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let check_locations = [
                (r, c - 1),
                (r, bound(c, cols-1)[1]),
                (bound(r, rows-1)[0], c),
                (bound(r, rows-1)[1], c),
                ];
            
            println!("{:?}", check_locations);

            if check_locations.into_iter()
                .all(|(i, j)| fuck_rust(&matrix, r, c) < fuck_rust(&matrix, i, j)) {
                mins.push(matrix[r as usize][c as usize] + 1);
            }
        }
    }

    println!("{:?}", mins);

    let part1 : int = mins.iter().sum();
    println!("{}", part1);
}