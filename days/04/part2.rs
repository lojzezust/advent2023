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

fn parse_card(line: &str) -> i32 {
    let card = line.split(": ").nth(1).unwrap();


    let mut split = card.split(" | ")
        .map(|s| split_into_numbers(s)); // split into numbers

    let winning = split.next().unwrap();
    let winning_set = winning.iter().cloned().collect::<HashSet<i32>>();
    // println!("{:?}", winning_set);

    let candidates = split.next().unwrap();
    // println!("{:?}", candidates);
    let mut wins = 0;
    for n in candidates {
        if winning_set.contains(&n){
            wins+=1;
        }
    }
    wins
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day04.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    // Parse number of wins for each card
    let mut wins_per_card: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        
        let wins = parse_card(&line);        
        wins_per_card.push(wins);
    }
    
    // Numbers of each cards held
    let mut num_cards: Vec<i32> = vec![1; wins_per_card.len()];

    let mut sum = 0;
    for i in 0..num_cards.len() {
        let num = num_cards[i];
        sum += num;
        let wins = wins_per_card[i];
        for j in 0..(wins as usize) {
            num_cards[i+j+1] += num;
        }
    }
    // println!("{:?}", queue);

    println!("{}", sum);

    Ok(())
}