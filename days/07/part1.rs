use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use counter::Counter;

fn card_val(c: char) -> i32{
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn hand_type(cards: Vec<i32>) -> Vec<i32> {
    let c: Counter<i32, i32> = Counter::init(cards.iter().cloned());
    
    // Counts of two most common cards determine the type
    let ord = c.most_common();
    let mut res:Vec<i32> = vec![ord[0].1, if ord.len() > 1 {ord[1].1} else {0}];
    
    // Remaining cards determine the tie-breaker
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
