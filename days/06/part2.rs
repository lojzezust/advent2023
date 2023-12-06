use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

// Extracts number from line (after removing whitespace)
fn read_line(s: &str) -> i64 {
    let mut s = String::from(&s[9..]);
    s.retain(|c| !c.is_whitespace());
    // println!("{}", s);
    s.parse::<i64>().unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day06.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    let times_l = lines.next().unwrap()?;
    let time = read_line(&times_l);
    
    let distances_l = lines.next().unwrap()?;
    let distance = read_line(&distances_l);

    // Brute force goes brrrr, O(n)
    // For larger inputs we could use binary search O(log n) 
    // or find analytical solution by solving quadratic equation O(1)
    let mut sum = 0;
    for i in 0..time {
        if i * (time - i) > distance {
            sum += 1;
        }
    }

    println!("{}", sum);

    Ok(())
}
