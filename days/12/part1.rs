use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn is_valid(pattern: &Vec<char>, nums: &Vec<i32>) -> bool{
    let mut arr: Vec<i32> = Vec::new();
    let mut gr = 0;
    for c in pattern.iter(){
        if *c == '#' {
            gr += 1;
        } else if gr > 0 {
            arr.push(gr);
            gr = 0;
        }
    }
    if gr>0 {
        arr.push(gr);
    }

    nums.eq(&arr)
}

// Brute force
fn find_ways(pattern: &mut Vec<char>, cur_i: usize, nums: &Vec<i32>) -> i32{
    if cur_i >= pattern.len(){
        return if is_valid(pattern, nums) {1} else {0};
    }

    if pattern[cur_i] == '?' {
        pattern[cur_i] = '.';
        let n1 = find_ways(pattern, cur_i+1, nums);
        pattern[cur_i] = '#';
        let n2 = find_ways(pattern, cur_i+1, nums);
        pattern[cur_i] = '?';
        n1 + n2
    } else {
        find_ways(pattern, cur_i+1, nums)
    }
}

fn main(){
    let f = File::open("inputs/day12.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Error parsing line.");
        let mut tokens = line.split(' ');
        let pattern = tokens.next().expect("Missing pattern");
        let nums: Vec<i32> = tokens.next().expect("Missing numbers").split(',').map(|x| x.parse().unwrap()).collect();

        let mut pattern = pattern.chars().collect();
        let n = find_ways(&mut pattern, 0, &nums);
        sum += n;
    }

    println!("Sum: {}", sum);
}
