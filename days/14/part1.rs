use std::{fs::File, io::BufReader};
use std::io::prelude::*;

type Matrix<T> = Vec<Vec<T>>;

#[derive(PartialEq)]
enum Tile {
    Rock,
    Obst,
    Empty
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Rock,
            '#' => Tile::Obst,
            _ => Tile::Empty
        }
    }

    /// Converts tile to char (debugging purposes)
    fn to_char(tile: &Self) -> char {
        match tile {
            Tile::Rock => 'O',
            Tile::Obst => '#',
            Tile::Empty => '.'
        }
    }
}

fn main() {
    let f = File::open("inputs/day14.txt").expect("Missing file");
    let reader = BufReader::new(f);

    // Read data
    let mut pat: Matrix<Tile> = Matrix::new();
    for line in reader.lines(){
        let line = line.expect("Error reading line.");
        
        if line.len() == 0 {
            break;
        }
        
        let tiles: Vec<Tile> = line.chars().map(Tile::from).collect();
        pat.push(tiles);
    }

    // Move north
    for col_i in 0..pat[0].len() {
        let mut li = 0;
        let mut ri = 1;
        while ri < pat.len() && li < pat.len(){
            if pat[li][col_i] != Tile::Empty {
                li += 1;
                continue;
            }
            if ri <= li {
                ri = li + 1;
                continue;
            }

            if pat[ri][col_i] == Tile::Rock {
                pat[li][col_i] = Tile::Rock;
                pat[ri][col_i] = Tile::Empty;
            } else if pat[ri][col_i] == Tile::Obst {
                li = ri;
            }
            ri += 1;
        }
    }

    // Compute load
    let n = pat.len();
    let mut total = 0;
    for (i, row) in pat.iter().enumerate() {
        for el in row.iter() {
            total += match el {
                Tile::Rock => n - i,
                _ => 0
            };
        }
    }

    println!("Total: {}", total);
}
