use std::collections::{BinaryHeap, HashSet};
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

type Matrix<T> = Vec<Vec<T>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Dir {Up, Right, Down, Left}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct State (
    i32, // Total loss
    Node
);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Node (
    usize, // i
    usize, // j
    Dir, // current dir
    usize // steps in current dir
);

fn allow_dir_part1(cur_dir: Dir, next_dir: Dir, l:usize) -> bool{
    cur_dir != next_dir || l < 3
}

fn allow_dir_part2(cur_dir: Dir, next_dir: Dir, l:usize) -> bool{
    (cur_dir == next_dir && l < 10) ||
    (cur_dir != next_dir && l >= 4)
}

fn dijkstra(map: &Matrix<i32>, part2: bool) -> i32{
    let (n,m) = (map.len(), map[0].len());
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut added: HashSet<State> = HashSet::new();
    let start_n = Node(0,0,Dir::Right,0);
    heap.push(State(0, start_n.clone()));
    visited.insert(start_n);

    // Function to check if new direction is allowed
    let allow_dir = if part2 {allow_dir_part2} else {allow_dir_part1};

    while !heap.is_empty() {
        let s = heap.pop().unwrap();
        // println!("{:?}", s);
        let State(c, Node(i,j, dir, l)) = s;
        visited.insert(s.1);

        if i == n-1 && j == m-1 {
            return -c;
        }

        // Add candidate nodes to PQ
        if i > 0 && dir != Dir::Down && allow_dir(dir, Dir::Up, l) {

            let new_l = if dir == Dir::Up {l+1} else {1};
            let new_n = Node(i-1, j, Dir::Up, new_l);
            let s = State(c - map[i-1][j], new_n.clone());
            if !visited.contains(&new_n) && !added.contains(&s) {
                added.insert(s.clone());
                heap.push(s);
            }
        }
        if j > 0 && dir != Dir::Right && allow_dir(dir, Dir::Left, l) {
            let new_l = if dir == Dir::Left {l+1} else {1};
            let new_n = Node(i, j-1, Dir::Left, new_l);
            let s = State(c - map[i][j-1], new_n.clone());
            if !visited.contains(&new_n) && !added.contains(&s) {
                added.insert(s.clone());
                heap.push(s);
            }
        }
        if i < n-1 && dir != Dir::Up && allow_dir(dir, Dir::Down, l) {
            let new_l = if dir == Dir::Down {l+1} else {1};
            let new_n = Node(i+1, j, Dir::Down, new_l);
            let s = State(c - map[i+1][j], new_n.clone());
            if !visited.contains(&new_n) && !added.contains(&s) {
                added.insert(s.clone());
                heap.push(s);
            }
        }
        if j < m-1 && dir != Dir::Left && allow_dir(dir, Dir::Right, l) {
            let new_l = if dir == Dir::Right {l+1} else {1};
            let new_n = Node(i, j+1, Dir::Right, new_l);
            let s = State(c - map[i][j+1], new_n.clone());
            if !visited.contains(&new_n) && !added.contains(&s) {
                added.insert(s.clone());
                heap.push(s);
            }
        }
    }
    -1
}


fn main() {
    let f = File::open("inputs/day17.txt").expect("Missing file");
    let reader = BufReader::new(f);

    // Read and parse map
    let mut map: Matrix<i32> = Matrix::new();
    for line in reader.lines(){
        let line = line.expect("Error reading line");
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        map.push(row);
    }

    let cost1 = dijkstra(&map, false);
    println!("Optimal cost (P1): {}", cost1);
    
    let cost2 = dijkstra(&map, true);
    println!("Optimal cost (P2): {}", cost2);
}
