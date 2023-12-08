use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day08.txt")?;
    // Read file line by line
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let inst: String = lines.next().unwrap().expect("Error reading line");
    lines.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in lines{
        let line = line.expect("Error reading line");
        let n = line[..3].to_string();
        let l = line[7..10].to_string();
        let r = line[12..15].to_string();
        let node = Node {left: l, right: r};

        nodes.insert(n, node);
    }

    
    let mut cur_node = String::from("AAA");
    let mut it = inst.chars().cycle();
    let mut steps = 0;
    // Follow and count instructions
    while cur_node != "ZZZ" {
        let n = &nodes[&cur_node];
        cur_node = match it.next().unwrap() {
            'R' => n.right.clone(),
            'L' => n.left.clone(),
            _ => cur_node
        };
        steps += 1;
    }

    println!("{steps}");

    Ok(())
}
