#![allow(warnings)]
//#[macro_use] extern crate scan_fmt;

use std::env;
use std::fs;

fn bsp(seat: &str) -> (i8, i8) {
    let (min, max) = (0, 127);

    if seat.chars().count() != 10 {
        return (-1, -1);
    }

    let row_bsp = &seat[0..7];
    let col_bsp = &seat[7..10];

    let mut row = 0;
    for r in row_bsp.chars() {
        row = (row << 1) +
            if r == 'B' { 1 } else { 0 };
    }
    let mut col = 0;
    for c in col_bsp.chars() {
        col = (col << 1) + 
            if c == 'R' { 1 } else { 0 };
    }

    (row, col)
}

fn seat_num(row: i8, col: i8) -> i16 {
    (row as i16 * 8 + col as i16)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() == 1 { "input.txt" } else { &args[1] };
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let seat_bsps : Vec<&str> = contents
        .lines()
        .collect();

    let mut seats = Vec::new();
    for seat in seat_bsps.iter() {
        let (row, col) = bsp(seat);

        seats.push(seat_num(row, col));
    }

    seats.sort();

    /*
    if let Some(highest) = seat_nums.pop() {
        println!("Part 1: {}", highest);
    }
    */

    for (i, s) in seats.iter().enumerate() {
        if i == 0 || i == seats.len()-1 {
            continue;
        }

        match seats[i+1] - s {
            1 => continue,
            2 => println!("{}", s + 1),
            _ => (),
        }
    }

    //println!("Part 2: {}", part2(forest)));
    //
}
