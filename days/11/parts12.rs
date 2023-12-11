use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let f = File::open("inputs/day11.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut stars: Vec<(usize,usize)> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    // Read the map and star locations
    for (y,line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");

        let mut row: Vec<char> = Vec::new();
        for (x,c) in line.chars().enumerate(){
            row.push(c);
            if c == '#' {
                stars.push((x,y));
            }
        }
        map.push(row);
    }

    // Number of added space (rows/cols)
    // let empty_space: i64 = 1; // Part 1
    let empty_space: i64 = 1_000_000 - 1; // Part 2

    // Check for empty columns and rows
    let mut row_map: Vec<i64> = Vec::new();
    let mut cur_i = 0;
    for i in 0..map.len() {
        if map[i].iter().all(|c| *c == '.') {
            cur_i += empty_space;
        }
        cur_i += 1;
        row_map.push(cur_i);
    }
    let mut col_map: Vec<i64> = Vec::new();
    let mut cur_j = 0;
    for j in 0..map[0].len() {
        if (0..map.len()).all(|i| map[i][j] == '.') {
            cur_j += empty_space;
        }
        col_map.push(cur_j);
        cur_j += 1;
    }

    // Remap star coordinates
    let stars: Vec<(i64, i64)> = stars.iter().map(|(x,y)| (col_map[*x], row_map[*y])).collect();
    // println!("{:?}", stars);

    // Sum of Manhattan distance between all pairs
    let mut total_distance = 0;
    for i in 0..stars.len(){
        for j in i+1..stars.len() {
            let (x1, y1) = stars[i];
            let (x2, y2) = stars[j];
            let dist = (x1 - x2).abs() + (y1 - y2).abs();
            total_distance += dist;
        }
    }

    println!("Total distance: {}", total_distance);
    Ok(())
}
