use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use counter::Counter;

fn card_val(c: char) -> i32{
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn hand_type(cards: Vec<i32>) -> Vec<i32> {
    // Only count non-jokers
    let cf: Vec<i32> = cards.iter().map(|v| *v).filter(|c| *c != 1).collect(); // Non-jokers
    let num_jokers = (cards.len() - cf.len()) as i32;
    let mut res = vec![0,0];
    if num_jokers < 5 {
        let c: Counter<i32, i32> = Counter::init(cf);
        let ord = c.most_common();
        res[0] = ord[0].1;
        res[1] = if ord.len() > 1 {ord[1].1} else {0};
    }

    // Add jokers to counts of leading two cards
    if res[0] + num_jokers > 5 {
        res[0] = 5;
        res[1] += num_jokers - (5 - res[0]);
    } else {
        res[0] += num_jokers;
    }
    
    // Add cards
    res.extend(cards);
    res
}


fn cmp_hands(s1: &str, s2: &str) -> std::cmp::Ordering {
    let c1: Vec<i32> = s1.chars().map(|c| card_val(c)).collect();
    let c2: Vec<i32> = s2.chars().map(|c| card_val(c)).collect();
    let t1 = hand_type(c1);
    let t2 = hand_type(c2);
    t1.cmp(&t2)
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: i32
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day07.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);

    let mut hands: Vec<Hand> = Vec::new();
    for line in reader.lines(){
        let line = line.expect("Error reading line");
        let mut split = line.split(" ");
        let hand = Hand {
            cards: String::from(split.next().unwrap()),
            bet: split.next().unwrap().parse::<i32>().unwrap(),
        };
        hands.push(hand);
    }

    hands.sort_by(|a, b| cmp_hands(&a.cards, &b.cards));
    let mut sum = 0;
    for (i,hand) in hands.iter().enumerate() {
        // println!("{:?}", &hand);
        sum += (i as i32 + 1) * hand.bet;
    }
    println!("{}", sum);

    Ok(())
}
