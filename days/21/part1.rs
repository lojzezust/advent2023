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
    
    // Parse modules
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

    let steps = 64;
    let mut parity: HashMap<(usize,usize), bool> = HashMap::new();
    while let Some((i,j,d)) = queue.pop_front() {
        // println!("{} {} {}", i, j, d);
        if d > steps || parity.contains_key(&(i,j)) {
            continue;
        }

        let pos = (i,j);
        parity.entry(pos).or_insert(d % 2 == 0);

        if i > 0 && map[i-1][j] == Tile::Empty && !parity.contains_key(&(i-1,j)) {
            queue.push_back((i-1,j,d+1));
        }
        if i < map.len()-1 && map[i+1][j] == Tile::Empty && !parity.contains_key(&(i+1,j)) {
            queue.push_back((i+1,j,d+1));
        }
        if j > 0 && map[i][j-1] == Tile::Empty && !parity.contains_key(&(i,j-1)) {
            queue.push_back((i,j-1,d+1));
        }
        if j < map[0].len()-1 && map[i][j+1] == Tile::Empty && !parity.contains_key(&(i,j+1)) {
            queue.push_back((i,j+1,d+1));
        }
    }

    let mut total = 0;
    for (_,v) in parity.iter() {
        if (steps % 2 == 0) == *v {
            total += 1;
        }
    }

    println!("Reachable: {:?}", total)

}
