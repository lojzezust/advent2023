use std::collections::HashMap;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

// Recursive subfunction
fn _find_ways(memo: &mut HashMap<(usize,usize,i32), i64>, pattern: &mut Vec<char>, nums: &Vec<i32>, cur_i:usize, cur_j:usize, cur_gr:i32) -> i64{
    let key = (cur_i, cur_j, cur_gr);
    if memo.contains_key(&key) {
        return memo[&key];
    }
    
    // Boundary condition
    if cur_i >= pattern.len(){
        // Check if final group size is satisfied
        let res = if cur_j == nums.len() - 1 && cur_gr == nums[cur_j] {
            1
        } else if cur_j >= nums.len() && cur_gr == 0 {
            1 
        } else {
            0
        };
        memo.insert(key, res);
        return res;
    }

    let res = if pattern[cur_i] == '?' {
        // If we choose '.', we need to check if the current group (j) was satisfied 
        let n1 = if cur_j < nums.len() && cur_gr > 0 && cur_gr == nums[cur_j] {
            pattern[cur_i] = '.';
            _find_ways(memo, pattern, nums, cur_i + 1, cur_j + 1, 0)
        } else if cur_gr == 0 {
            _find_ways(memo, pattern, nums, cur_i + 1, cur_j, 0)
        } else {0};
        pattern[cur_i] = '#';
        let n2 = _find_ways(memo, pattern, nums,  cur_i + 1, cur_j, cur_gr + 1);
        pattern[cur_i] = '?';
        n1 + n2
    } else if pattern[cur_i] == '#' {
        _find_ways(memo, pattern, nums, cur_i+1, cur_j, cur_gr + 1)
    }
    // The current character is '.' check if the current group (j) was satisfied and recurse accordingly 
    else if cur_j < nums.len() && cur_gr > 0 && cur_gr == nums[cur_j] {
        _find_ways(memo, pattern, nums, cur_i+1, cur_j+1, 0)
    } else if cur_gr == 0 {
        _find_ways(memo, pattern, nums, cur_i+1, cur_j, 0)
    } else {
        0
    };

    memo.insert(key, res);
    res
}

// Calls recursion with memoization (DP). 
fn find_ways(pattern: &mut Vec<char>, nums: &Vec<i32>) -> i64 {
    // Partial solutions with starting index i, current group j and group size gr (number of leading #) are cached
    let mut memo: HashMap<(usize,usize,i32), i64> = HashMap::new();
    _find_ways(&mut memo, pattern, nums, 0, 0, 0)
}

fn main(){
    let f = File::open("inputs/day12.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.expect("Error parsing line.");
        let mut tokens = line.split(' ');

        let pattern = tokens.next().expect("Missing pattern");
        let mut pattern = (String::from(pattern) + "?").repeat(5);
        pattern.pop(); // Remove last ?

        let nums: Vec<i32> = tokens.next().expect("Missing numbers").split(',').map(|x| x.parse().unwrap()).collect();
        let nums = nums.repeat(5);

        let mut pattern = pattern.chars().collect();
        let n = find_ways(&mut pattern, &nums);

        // println!("{n}");

        sum += n;
    }

    println!("Sum: {}", sum);
}
