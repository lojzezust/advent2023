use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;


fn main() -> io::Result<()> {
    let f = File::open("inputs/day09.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut future = 0;
    let mut past = 0;
    for line in reader.lines(){
        let line = line.expect("Error reading line");
        let seq: Vec<i32> = line.split(' ').map(|x| x.parse().expect("Error parsing int")).collect();

        let mut seqs: Vec<Vec<i32>> = vec![seq];
        loop {
            let cur_seq = seqs.last().expect("No elements in seqs");
            let mut new_seq = Vec::new();
            let mut all0 = true;
            for (x1,x2) in zip(cur_seq.iter(), cur_seq.iter().skip(1)) {
                let diff = x2-x1;
                if diff != 0 {
                    all0 = false;
                }
                new_seq.push(diff);
            }

            seqs.push(new_seq);
            if all0 {
                break;
            }
        }

        // Predict the future (Part 1)
        let mut diff = *seqs.last().unwrap().last().expect("Last seq is empty");
        for seq in seqs.iter().rev().skip(1){
            let last = *seq.last().unwrap();
            diff += last;
        }
        future += diff;

        // Interpolate the past (Part 2)
        let mut diff = *seqs.last().unwrap().first().expect("Last seq is empty");
        for seq in seqs.iter().rev().skip(1){
            let first = *seq.first().unwrap();
            diff = first - diff;
        }
        past += diff;
    }

    println!("Future: {}", future);
    println!("Past: {}", past);
    Ok(())
}
