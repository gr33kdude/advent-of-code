#![allow(warnings)]

#[macro_use] extern crate scan_fmt;

use std::{env,fs,io};
use std::cmp::{min,max,Ordering};

type int = u32;

#[derive(Copy, Clone, Debug)]
enum FoldDirection {
    Up(int),
    Left(int),
}

enum ParseState {
    Coords,
    Folding,
}

fn apply_fold(coords : &mut [Coord], size : &mut Coord, dir : FoldDirection) -> () {
    for coord in coords {
        match dir {
            FoldDirection::Up(amt) => {
                if amt < coord.row && coord.row <= 2*amt {
                    coord.row -= 2*(coord.row - amt);
                }
            },
            FoldDirection::Left(amt) => {
                if amt < coord.col && coord.col <= 2*amt {
                    coord.col -= 2*(coord.col - amt);
                }
            },
        }
    }

    match dir {
        FoldDirection::Up(amt) => size.row = size.row / 2,
        FoldDirection::Left(amt) => size.col = size.col / 2,
    };
}

fn image_size(coords : &[Coord]) -> Coord {
    let mut max = Coord{row: 0, col: 0};
    for Coord{row, col} in coords {
        max.row = std::cmp::max(max.row, row+1);
        max.col = std::cmp::max(max.col, col+1);
    }

    max
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");
    let lines = input
        .trim_end()
        .lines();

    // Parse input
    let mut coords : Vec<Coord> = Vec::new();
    let mut folds : Vec<FoldDirection> = Vec::new();
    let mut state : ParseState = ParseState::Coords;
    for line in lines {
        if line.is_empty() {
            state = ParseState::Folding;
            continue;
        }

        match state {
            ParseState::Coords => {
                if let Ok((col, row)) = scan_fmt!(line, "{d},{d}", int, int) {
                    coords.push(Coord{row, col});
                }
            }
            ParseState::Folding => {
                match scan_fmt!(line, "fold along {[xy]}={d}", char, int).unwrap() {
                    ('x', value) => folds.push(FoldDirection::Left(value)),
                    ('y', value) => folds.push(FoldDirection::Up(value)),
                    _ => panic!("Invalid fold instruction!")
                };
            }
        }
    }

    // find image bounds
    let mut size = image_size(&coords);

    // Sort coordinates
    coords.sort();

    for fold in folds.iter() {
        apply_fold(&mut coords, &mut size, *fold);

        // make sure to sort and dedup after each fold
        coords.sort();
        coords.dedup();
    }
    println!("");

    print_image(&coords, &size);

    println!("Num colored pixels: {}", coords.len());
}

#[derive(Copy, Clone, Debug)]
struct Coord {
    row : int,
    col : int,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (Ordering::Equal, y_ord) => y_ord,
            (x_ord, _) => x_ord,
        }
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.col) == (other.row, other.col)
    }
}
impl Eq for Coord {}

fn catch_up(start: &Coord, end : &Coord, max : &Coord) -> () {
    if (start.row < end.row) {
        // finish the line
        for _ in start.col..max.col {
            print!(".");
        }
        println!("");

        // print any intervening lines
        for _ in start.row+1..end.row {
            for _ in 0..max.col {
                print!(".");
            }
            println!("");
        }
    }

    let start_col = if start.row == end.row { start.col } else { 0 };

    // go up-to current point
    for _ in start_col..end.col {
        print!(".");
    }
}

fn print_image(coords : &[Coord], max : &Coord) -> () {
    let limit = Coord{row: max.row-1, col: max.col};
    let mut cur : Coord = Coord{row: 0, col: 0};
    for next in coords {
        // catch up to the current coord, row first then col
        if cur <= *next {
            catch_up(&cur, &next, &max);
            print!("#");
        }
        
        if *next >= limit {
            println!("WTF: next = {:?}, limit = {:?}", next, limit);
        }

        // prepare the current position 
        cur = Coord{row: next.row, col: next.col+1};
    }

    if cur < limit {
        catch_up(&cur, &limit, &max);
        println!();
    }
}