use std::{fs::File, io::BufReader};
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up, Right, Down, Left
}

impl Dir {
    fn from(s: &str)->Self {
        match s {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("Unknown dir")
        }
    }

    fn from_i(i: i32)->Self {
        match i {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("Unknown dir")
        }
    }

    fn to_coords(&self) -> (i64,i64) {
        match self {
            Dir::Up => (0,-1),
            Dir::Right => (1,0),
            Dir::Down => (0,1),
            Dir::Left => (-1,0)
        }
    }
}

fn parse_hex(s: &str) -> (Dir, i64) {
    let l = i64::from_str_radix(&s[..5], 16).unwrap();
    let dir = Dir::from_i(s[5..].parse().unwrap());
    (dir, l)
}

fn main() {
    for part2 in [false, true] {
        let f = File::open("inputs/day18.txt").expect("Missing file");
        let reader = BufReader::new(f);
        
        // Follow instructions to build map
        let (mut x, mut y) = (0,0);
        
        let mut boundary: i64 = 0;
        let mut area: i64 = 0;
        for line in reader.lines(){
            let line = line.expect("Error reading line");
            let mut it = line.split(' ');
            let (dir, l) = if part2 {
                parse_hex(&it.nth(2).unwrap()[2..8])
            } else {
                let dir = it.next().unwrap();
                let l: i64 = it.next().unwrap().parse().unwrap();
                (Dir::from(dir), l)
            };

            let (dx,dy) = dir.to_coords();
            (x,y) = (x + l * dx, y + l * dy);

            if let Dir::Right = dir {
                area += l * y;
            } else if let Dir::Left = dir {
                area -= l * y;
            }
            boundary += l;
        }
        area = area.abs();

        println!("Part {}:", if part2 {2} else {1});
        println!("  - 0-width area: {}, boundary: {}", area, boundary);
        println!("  - Adjusted area: {}", area + boundary / 2 + 1);
    }
    
}

