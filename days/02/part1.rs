use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let f = File::open("inputs/day02.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut sum = 0;
    for (i,line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                let mut rounds = l.split(": ")
                    .nth(1)
                    .unwrap()
                    .split("; ");
                
                // Check if all rounds are valid
                let ok = rounds.all(|round| -> bool {

                    // Check if all numbers of balls are valid
                    round.split(", ").all(|ball| {
                        let s: Vec<&str> = ball.split(" ").take(2).collect();
                        let col = s[1];
                        let n = s[0].parse::<i32>().unwrap();
                        
                        match col {
                            "red" => if n > 12 { false } else {true},
                            "green" => if n > 13 { false } else {true},
                            "blue" => if n > 14 { false } else {true},
                            _ => false,
                        }

                    })
                });

                if ok {
                    sum += i+1;
                }

            },
            Err(e) => println!("{}", e),
        }
    }

    println!("{}", sum);

    Ok(())
}