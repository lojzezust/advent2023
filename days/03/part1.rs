use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max,min};

fn check_neighbors(M: &Vec<String>, i: usize, j: usize) -> bool {
    // println!("Checking neighbors for ({}, {})", i, j);
    for k in max(0, (i as i32)-1)..min(M.len() as i32, (i as i32)+2) {
        for l in max(0, (j as i32)-1)..min(M[i].len() as i32, (j as i32)+2) {
            // println!("    ({}, {})", k, l);
            let c = M[usize::try_from(k).unwrap()].chars().nth(usize::try_from(l).unwrap()).unwrap();
            if c != '.' && !c.is_digit(10) {
                return true;
            }
        }
    }
    return false;
}

fn find_numbers(M: &Vec<String>) -> i32{
    let mut total = 0;
    for i in 0..M.len() {
        let mut num = String::new();
        let mut valid = false;
        for j in 0..M[i].len() {
            let c = M[i].chars().nth(j).unwrap();
            if c.is_digit(10) {
                num.push(c);
                if check_neighbors(&M, i, j) {
                    valid = true;
                }
            } else {
                if valid && num.len() > 0 {
                    total += num.parse::<i32>().unwrap();
                    // println!("Found valid number: {}", num);
                } else if  (num.len() > 0) {
                    // println!("Found invalid number (:{}): {}", i, num);
                }
                num = String::new();
                valid = false;
            }
        }
    }

    total
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day03.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut M: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => M.push(l + "."),
            Err(e) => println!("Error: {}", e),
        };
    }

    let res = find_numbers(&M);
    println!("{res}");

    Ok(())
}
