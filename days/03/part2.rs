use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max,min};
use std::collections::{HashMap,HashSet};

#[derive(Eq,PartialEq,Hash,Debug,Clone)]
struct Gear {
    i: usize,
    j: usize
}

fn find_neighbor_gears(M: &Vec<String>, i: usize, j: usize) -> HashSet<Gear> {
    // println!("Checking neighbors for ({}, {})", i, j);
    let mut gears = HashSet::new();
    for k in max(0, (i as i32)-1)..min(M.len() as i32, (i as i32)+2) {
        for l in max(0, (j as i32)-1)..min(M[i].len() as i32, (j as i32)+2) {
            // println!("    ({}, {})", k, l);
            let ku = usize::try_from(k).unwrap();
            let lu = usize::try_from(l).unwrap();
            let c = M[ku].chars().nth(lu).unwrap();
            if c == '*' {
                gears.insert(Gear{i: ku, j: lu});
            }
        }
    }
    
    gears
}

fn find_numbers(M: &Vec<String>) -> i32{
    // Map of Gear (locations) to list of neighboring numbers
    let mut gear_map: HashMap<Gear, Vec<i32>> = HashMap::new();
    for i in 0..M.len() {
        let mut num = String::new();
        // Set of Gear (locations) that are neighbors
        let mut gears = HashSet::new();
        for j in 0..M[i].len() {
            let c = M[i].chars().nth(j).unwrap();
            if c.is_digit(10) {
                num.push(c);
                gears = gears.union(&find_neighbor_gears(&M, i, j)).cloned().collect();
            } else {
                // End of number
                for gear in gears.iter() {
                    if !gear_map.contains_key(gear){
                        gear_map.insert(gear.clone(), Vec::new());
                    }
                    gear_map.get_mut(gear).unwrap().push(num.parse::<i32>().unwrap());
                }
                num = String::new();
                gears = HashSet::new();
            }
        }
    }

    let mut total = 0;
    for (gear, nums) in gear_map.iter() {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
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
