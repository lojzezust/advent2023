use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, VecDeque};

type Matrix<T> = Vec<Vec<T>>;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Rock,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' | 'S' => Tile::Empty,
            '#' => Tile::Rock,
            _ => panic!("Invalid tile")
        }
    }
}

fn main() {
    let f = File::open("inputs/day21.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse map
    let mut map: Matrix<Tile> = Matrix::new();
    let mut queue = VecDeque::new();
    for (i,line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");
        let mut row: Vec<Tile> = Vec::new();
        for (j,c) in line.chars().enumerate() {
            if c == 'S' {
                queue.push_back((i,j,0));
            }
            row.push(Tile::from_char(c));
        }
        map.push(row);
    }

    let steps = 200; // To cover entire map
    let mut dist: HashMap<(usize,usize), i32> = HashMap::new();
    while let Some((i,j,d)) = queue.pop_front() {
        // println!("{} {} {}", i, j, d);
        if d > steps || dist.contains_key(&(i,j)) {
            continue;
        }

        let pos = (i,j);
        dist.entry(pos).or_insert(d);

        if i > 0 && map[i-1][j] == Tile::Empty && !dist.contains_key(&(i-1,j)) {
            queue.push_back((i-1,j,d+1));
        }
        if i < map.len()-1 && map[i+1][j] == Tile::Empty && !dist.contains_key(&(i+1,j)) {
            queue.push_back((i+1,j,d+1));
        }
        if j > 0 && map[i][j-1] == Tile::Empty && !dist.contains_key(&(i,j-1)) {
            queue.push_back((i,j-1,d+1));
        }
        if j < map[0].len()-1 && map[i][j+1] == Tile::Empty && !dist.contains_key(&(i,j+1)) {
            queue.push_back((i,j+1,d+1));
        }
    }

    // Number of steps is nice: we will reach the end of exactly 202300 full squares if we go straight
    let total_steps: i32 = 26501365;
    let n_squares: i64 = 202300; // 26501365 = 202300 * 131 + 65 (center square)

    let parity = total_steps % 2;
    let num_normal = dist.values().filter(|x| **x % 2 == parity).count() as i64;
    let num_inverted = dist.values().filter(|x| **x % 2 != parity).count() as i64;
    let num_normal_corner = dist.values().filter(|x| **x % 2 == parity && **x > 65).count() as i64;
    let num_inverted_corner = dist.values().filter(|x| **x % 2 != parity && **x > 65).count() as i64;
    println!("(Full) Num normal: {}, Num inverted: {}", num_normal, num_inverted);
    println!("(Corners) Num normal: {}, Num inverted: {}", num_normal_corner, num_inverted_corner);


    // Compute number of full squares we will cover
    let num_full_squares_n = (n_squares - 1).pow(2); // Normal
    let num_partial_squares_n = (n_squares + 1).pow(2) - num_full_squares_n;
    let num_full_squares_i = (n_squares).pow(2); // Inverted

    // Compute number of corners we will reach for each type
    let num_corners_n = n_squares + 1;
    let num_corners_i = n_squares;

    // Add full squares and corners
    let total_full = num_full_squares_n * num_normal + num_full_squares_i * num_inverted;
    let total_corners = num_partial_squares_n * num_normal - num_corners_n * num_normal_corner + num_corners_i * num_inverted_corner;

    let total = total_full + total_corners;

    println!("Total: {}", total);
}
