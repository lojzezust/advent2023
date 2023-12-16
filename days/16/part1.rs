use std::collections::VecDeque;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

type Matrix<T> = Vec<Vec<T>>;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Dir {Left, Right, Up, Down}

impl Dir {
    fn invert(&self) -> Self {
        match self {
            Dir::Left => {Dir::Right},
            Dir::Right => {Dir::Left},
            Dir::Up => {Dir::Down},
            Dir::Down => {Dir::Up}
        }
    }
}

#[derive(Clone)]
enum Field {
    Empty,
    Splitter{vert:bool},
    Mirror{right:bool}
}

impl Field {
    fn process_beam(&self, Beam{x,y,dir}:Beam) -> Vec<Beam> {
        let mut out_dirs: Vec<Dir> = Vec::new();
        match self {
            Field::Mirror{right} => {
                println!("Reflected!");
                let mut out_d = match dir {
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                    Dir::Up => {Dir::Left},
                    Dir::Down => {Dir::Right}
                };
                if *right {
                    out_d = out_d.invert();
                }
                out_dirs.push(out_d);
            },
            Field::Splitter{vert} => {
                match (dir, vert) {
                    (Dir::Left | Dir::Right, true) => {
                        println!("Split!");
                        out_dirs.push(Dir::Up); 
                        out_dirs.push(Dir::Down);
                    },
                    (Dir::Up | Dir::Down, false) => {
                        println!("Split!");
                        out_dirs.push(Dir::Left); 
                        out_dirs.push(Dir::Right);
                    }
                    _ => {out_dirs.push(dir);}
                };
            },
            Field::Empty => {out_dirs.push(dir);}
        }

        let mut out: Vec<Beam> = Vec::new();
        for d in out_dirs {
            out.push(match d {
                Dir::Left => Beam{x: x-1 ,y: y, dir: d},
                Dir::Right => Beam{x: x+1 ,y: y, dir: d},
                Dir::Up => Beam{x: x ,y: y-1, dir: d},
                Dir::Down => Beam{x: x ,y: y+1, dir: d},
            });
        }
        out
    }

    fn from(c:char) -> Self {
        match c {
            '/' => Field::Mirror { right: true },
            '\\' => Field::Mirror { right: false },
            '-' => Field::Splitter { vert: false },
            '|' => Field::Splitter { vert: true },
            _ => Field::Empty,
        }
    }
}

#[derive(Debug)]
struct Beam{x:i32,y:i32,dir:Dir}

fn main() {
    let f = File::open("inputs/day16.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let mut map: Matrix<Field> = Matrix::new();

    for line in reader.lines(){
        let line = line.expect("Error reading line");
        let mut row: Vec<Field> = Vec::new();
        for c in line.chars() {
            row.push(Field::from(c));
        }
        map.push(row);
    }

    let (n,m) = (map.len(), map[0].len());
    let mut visited: Matrix<Vec<Dir>> = vec![vec![Vec::new();m]; n];
    let mut queue: VecDeque<Beam> = VecDeque::new();
    queue.push_back(Beam{x:0, y:0, dir:Dir::Right});
    while !queue.is_empty() {
        let beam = queue.pop_front().unwrap();
        println!("{:?}", beam);
        if beam.x < 0 || beam.y < 0 || beam.x >= m as i32 || beam.y >= n as i32 {
            continue
        }

        let (x,y) = (beam.x as usize, beam.y as usize);
        if visited[y][x].contains(&beam.dir) {
            continue;
        }

        visited[y][x].push(beam.dir);
        let field = &map[y][x];

        let out_beams = field.process_beam(beam);
        for b in out_beams {
            queue.push_back(b);
        }
    }

    let mut total: i32 = 0;
    for row in visited {
        total += row.iter()
            .map(|x| if x.len() > 0 {1} else {0})
            .sum::<i32>();
    } 

    println!("Sum: {}", total);
}
