use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn split_into_numbers(s: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut i=0;
    while i < s.len() {
        let s = &s[i..i+2];
        numbers.push(s.trim().parse::<i32>().unwrap());
        i += 3;
    }

    numbers
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day04.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let card = line.split(": ").nth(1).unwrap();


        let mut split = card.split(" | ")
            .map(|s| split_into_numbers(s)); // split into numbers

        let winning = split.next().unwrap();
        let winning_set = winning.iter().cloned().collect::<HashSet<i32>>();
        // println!("{:?}", winning_set);

        let candidates = split.next().unwrap();
        // println!("{:?}", candidates);
        let mut winpow = 0;
        for n in candidates {
            if winning_set.contains(&n) {
                winpow = if winpow == 0 {1} else {winpow * 2};
            }
        }
        sum += winpow;
    }

    println!("{}", sum);

    Ok(())
}