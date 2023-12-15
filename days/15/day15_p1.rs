use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn hash(string: &str) -> i32 {
    let mut hash = 0;
    for c in string.chars() {
        let ascii = c as i32;
        hash = ((hash + ascii) * 17) % 256;
    }
    hash
}

fn main() {
    let f = File::open("inputs/day15.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let line = reader.lines().next().unwrap().expect("Error reading line");

    let mut sum: i32 = 0;
    for cmd in line.split(',') {
        sum += hash(cmd);
        // println!("{}: {}", cmd, hash(cmd));
    }
    println!("Sum: {}", sum);
}
