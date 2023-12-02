use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

fn process_game(game: &str) -> i32{
    let rounds = game.split(": ")
                    .nth(1)
                    .unwrap()
                    .split("; ");

    let mut cmax:[i32; 4] = [0,0,0,0];

    for round in rounds {
        for ball in round.split(", ") {
            let s: Vec<&str> = ball.split(" ").take(2).collect();
            let col = s[1];
            let n = s[0].parse::<i32>().unwrap();
            let ci = match col {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => 3,
            };
            cmax[ci] = max(cmax[ci], n);
        }
    }

    cmax[0] * cmax[1] * cmax[2]
}

fn main() -> io::Result<()> {
    let mut f = File::open("inputs/day02.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut sum = 0;
    for (i,line) in reader.lines().enumerate() {
        let power = match line {
            Ok(l) => process_game(&l),
            Err(e) => 0,
        };
        sum += power;
    }

    println!("{}", sum);

    Ok(())
}