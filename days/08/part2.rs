use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use num::integer::lcm;


#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

fn cycle_len(inst: &String, nodes: &HashMap<String, Node>, start_node: String) -> i64{
    let mut cur_node = start_node.clone();
    let mut it = inst.chars().cycle();
    let mut steps = 0;
    while !cur_node.ends_with("Z") {
        let n = &nodes[&cur_node];
        cur_node = match it.next().unwrap() {
            'R' => n.right.clone(),
            'L' => n.left.clone(),
            _ => cur_node
        };
        steps += 1;
    };
    steps
}

fn main() -> io::Result<()> {
    let f = File::open("inputs/day08.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let inst: String = lines.next().unwrap().expect("Error reading line");
    lines.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut start_nodes: Vec<String> = Vec::new();
    for line in lines{
        let line = line.expect("Error reading line");
        let n = line[..3].to_string();
        let l = line[7..10].to_string();
        let r = line[12..15].to_string();
        let node = Node {left: l, right: r};

        // Store all starting nodes
        nodes.insert(n.clone(), node);
        if n.ends_with("A") {
            start_nodes.push(n);
        }
    }

    // Compute cycle lengths for each start node and compute LCM (when the cycles will synchronize)
    let cycles = start_nodes.iter().map(|n| cycle_len(&inst, &nodes, n.clone()));
    let res = cycles.fold(1, |acc, x| lcm(acc, x));

    println!("{res}");

    Ok(())
}
