use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

struct Item {
    x: usize,
    y: usize,
    dist: usize,
    from: Dir,
}

#[derive(PartialEq)]
enum Dir {
    T, R, B, L, None
}

struct Pipe {
    t: bool,
    r: bool,
    b: bool,
    l: bool,
    left_turn: i32, // 0 - no change, 1 - turns left, 2 - turns right
    dir: Dir, // incoming direction. If coming from the other side, turn count and seed points need to be inverted
    seed_l: Vec<(i32,i32)>, // Seed points on the left of the pipe
    seed_r: Vec<(i32,i32)> // Seed points on the right of the pipe
}

impl Pipe {
    fn empty() -> Self {
        Pipe {t: false, r: false, b: false, l: false, left_turn: 0, dir:Dir::None, seed_l: Vec::new(), seed_r: Vec::new()}
    }

    fn from(c: char) -> Self {
        match c {
            '|' => Pipe {t: true, r: false, b: true, l: false, left_turn: 0, dir:Dir::B, seed_l: vec![(-1,0)], seed_r: vec![(1,0)]},
            '-' => Pipe {t: false, r: true, b: false, l: true, left_turn: 0, dir:Dir::L, seed_l: vec![(0,-1)], seed_r: vec![(0,1)]},
            'L' => Pipe {t: true, r: true, b: false, l: false, left_turn: -1, dir:Dir::R, seed_l: vec![(-1,0), (0,1)], seed_r: vec![(1,0),(0,-1)]},
            'J' => Pipe {t: true, r: false, b: false, l: true, left_turn: 1, dir:Dir::L, seed_l: vec![(-1,0), (0,-1)], seed_r: vec![(1,0),(0,1)]},
            '7' => Pipe {t: false, r: false, b: true, l: true, left_turn: -1, dir:Dir::L, seed_l: vec![(1,0),(0,-1)], seed_r: vec![(-1,0),(0,1)]},
            'F' => Pipe {t: false, r: true, b: true, l: false, left_turn: -1, dir:Dir::B, seed_l: vec![(-1,0),(0,-1)], seed_r: vec![(1,0),(0,1)]},
            'S' => Pipe {t: true, r: true, b: true, l: true, left_turn: 0, dir:Dir::None, seed_l: Vec::new(), seed_r: Vec::new()},
            _ => Self::empty()
        }
    }

    fn get_seeds(&self, from_dir: &Dir) -> (&Vec<(i32,i32)>, &Vec<(i32,i32)>){
        // Get left and right seed points (neighbors) depending on the incoming direction
        if self.dir == *from_dir {
            (&self.seed_l, &self.seed_r)
        } else {
            (&self.seed_r, &self.seed_l)
        }
    }

    fn get_left_turns(&self, from_dir: &Dir) -> i32 {
        // Get left turn counter change depending on the incoming direction
        if self.dir == *from_dir {
            self.left_turn
        } else {
            -self.left_turn
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day10.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);


    let mut map: Vec<Vec<char>> = Vec::new();
    let mut queue: VecDeque<Item> = VecDeque::new();
    for (i,line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");

        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                queue.push_back(Item {x:j, y:i, dist:0, from:Dir::None});
            }
            row.push(c);
        }
        map.push(row);
    }

    // BFS track the boundary
    let mut left_turns: i32 = 0;
    let mut left_seeds: VecDeque<(i32, i32)> = VecDeque::new();
    let mut right_seeds: VecDeque<(i32, i32)> = VecDeque::new();
    while !queue.is_empty() {
        let Item{x, y, dist:d, from} = queue.pop_front().unwrap();

        let cur = map[y][x];
        let pipe = Pipe::from(cur);
        if pipe.t && y > 0 && Pipe::from(map[y-1][x]).b {
            queue.push_back(Item{x:x, y:y-1, dist:d+1, from: Dir::B});
        }
        if pipe.l && x > 0 && Pipe::from(map[y][x-1]).r {
            queue.push_back(Item{x:x-1, y:y, dist:d+1, from: Dir::R});
        }
        if pipe.b && y < map.len() && Pipe::from(map[y+1][x]).t {
            queue.push_back(Item{x:x, y:y+1, dist:d+1, from: Dir::T});
        }
        if pipe.r && x < map[0].len() && Pipe::from(map[y][x+1]).l {
            queue.push_back(Item{x:x+1, y:y, dist:d+1, from: Dir::L});
        }
        map[y][x] = '+';

        // Track number of left and right turns
        left_turns += pipe.get_left_turns(&from);

        if cur == 'S' {
            while queue.len() > 1 {
                queue.pop_front(); // Only inspect one path
            }
        }

        // Add seed points to the left and to the right of the traveling direction
        let (ls,rs) = pipe.get_seeds(&from);
        let x = x as i32;
        let y = y as i32;
        for (lx,ly) in ls {
            left_seeds.push_back((x+lx, y+ly));
        }
        for (rx,ry) in rs {
            right_seeds.push_back((x+rx, y+ry));
        }
    }

    println!("Left turns: {}", left_turns);
    println!("Left seeds: {}, Right seeds: {}", left_seeds.len(), right_seeds.len());

    // DFS fill from seeds
    let mut seeds = if left_turns > 0 {left_seeds} else {right_seeds};
    let mut count = 0;
    while !seeds.is_empty(){
        let (x,y) = seeds.pop_front().unwrap();
        if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
            continue
        }
        let (x,y) = (x as usize, y as usize);
        let c = map[y][x];
        if c == '+' || c == 'X' {
            continue
        }

        if y > 0 {
            seeds.push_front((x as i32, y as i32 - 1));
        }
        if x > 0 {
            seeds.push_front((x as i32 - 1, y as i32));
        }
        if y <= map.len() {
            seeds.push_front((x as i32, y as i32 + 1));
        }
        if x <= map[0].len() {
            seeds.push_front((x as i32 + 1, y as i32));
        }

        count += 1;
        map[y][x] = 'X';
    }

    // Print map (for verification)
    // for line in &map {
    //     println!("{}", String::from_iter(line.iter()));
    // }

    println!("Interior area: {}", count);
    Ok(())
}
