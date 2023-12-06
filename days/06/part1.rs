use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

// Reads line and returns vector of numbers
fn read_line(s: &str) -> Vec<i32> {
    let tokens = s.split_whitespace().skip(1);
    let numbers: Vec<i32> = tokens.map(|t| t.parse::<i32>().unwrap()).collect();
    numbers
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day06.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    let times_l = lines.next().unwrap()?;
    let times = read_line(&times_l);
    
    let distances_l = lines.next().unwrap()?;
    let distances = read_line(&distances_l);

    let mut prod = 1;
    for (time, dist) in zip(times, distances) {
        // Check all possible holding times
        let mut sum = 0;
        for i in 0..time {
            if i * (time - i) > dist {
                sum += 1;
            }
        }
        prod *= sum;
    }

    println!("{}", prod);

    Ok(())
}
