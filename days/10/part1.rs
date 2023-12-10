use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

struct Item {
    x: usize,
    y: usize,
    dist: usize // Current distance from the start
}

struct Pipe {
    t: bool,
    r: bool,
    b: bool,
    l: bool
}

impl Pipe {
    fn empty() -> Self {
        Pipe {t: false, r: false, b: false, l: false}
    }

    fn from(c: char) -> Self {
        match c {
            '|' => Pipe {t: true, r: false, b: true, l: false},
            '-' => Pipe {t: false, r: true, b: false, l: true},
            'L' => Pipe {t: true, r: true, b: false, l: false},
            'J' => Pipe {t: true, r: false, b: false, l: true},
            '7' => Pipe {t: false, r: false, b: true, l: true},
            'F' => Pipe {t: false, r: true, b: true, l: false},
            'S' => Pipe {t: true, r: true, b: true, l: true},
            _ => Self::empty()
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day10.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);


    let mut map: Vec<Vec<Pipe>> = Vec::new();
    let mut queue: VecDeque<Item> = VecDeque::new();
    for (i,line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");

        let mut row: Vec<Pipe> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                queue.push_back(Item {x:j, y:i, dist:0});
            }
            row.push(Pipe::from(c));
        }
        map.push(row);
    }

    // BFS (flood fill across the pipe in both directions)
    let mut max_dist = 0;
    while !queue.is_empty() {
        let Item{x, y, dist:d} = queue.pop_front().unwrap();
        max_dist = std::cmp::max(max_dist, d);

        let cur = &map[y][x];
        if cur.t && y > 0 && map[y-1][x].b {
            queue.push_back(Item{x:x, y:y-1, dist:d+1});
        }
        if cur.l && x > 0 && map[y][x-1].r {
            queue.push_back(Item{x:x-1, y:y, dist:d+1});
        }
        if cur.b && y < map.len() && map[y+1][x].t {
            queue.push_back(Item{x:x, y:y+1, dist:d+1});
        }
        if cur.r && x < map[0].len() && map[y][x+1].l {
            queue.push_back(Item{x:x+1, y:y, dist:d+1});
        }
        map[y][x] = Pipe::empty();
    }

    println!("Max distance: {}", max_dist);
    Ok(())
}
