use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::{min,max,Ordering};
use std::collections::HashSet;

#[derive(Debug)]
struct Block {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl Block {
    fn from_str(s: &str) -> Block {
        let mut it = s.split("~");
        let left: Vec<usize> = it.next().expect("Missing left").split(',').map(|x| x.parse().unwrap()).collect();
        let right: Vec<usize> = it.next().expect("Missing right").split(',').map(|x| x.parse().unwrap()).collect();
        Block{x: (left[0],right[0]), y: (left[1],right[1]), z: (left[2],right[2])}
    }
}

/// Find the lowest z value where the block can fall, and the indices of the blocks that support it
fn find_fall_place(stack: &Vec<Block>, block: &Block) -> (usize, Vec<usize>) {
    let mut lowest = 0;
    let mut support: Vec<usize> = Vec::new();
    for (i,ex) in stack.iter().enumerate() {
        let intersects_x = max(block.x.0, ex.x.0) <= min(block.x.1, ex.x.1);
        let intersects_y = max(block.y.0, ex.y.0) <= min(block.y.1, ex.y.1);
        if intersects_x && intersects_y {
            match (ex.z.1 + 1).cmp(&lowest) {
                Ordering::Equal => support.push(i),
                Ordering::Greater => {
                    lowest = ex.z.1 + 1;
                    support.clear();
                    support.push(i);
                },
                Ordering::Less => (),
            }
        }
    }
    (lowest, support)
}

fn main() {
    let f = File::open("inputs/day22.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse map
    let mut blocks = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let block = Block::from_str(&line);
        blocks.push(block);
    }

    // Sort by z
    blocks.sort_by(|a,b| a.z.cmp(&b.z));

    let mut blocks_fallen = Vec::new();
    let mut disintegratable: HashSet<usize> = HashSet::new();
    for (i,block) in blocks.iter().enumerate() {
        let Block{x: _, y: _, z: (z0,z1)} = block;
        let (z, support) = find_fall_place(&blocks_fallen, &block);

        // If block has a single support block, the support block is not disintegratable
        if support.len() == 1 {
            disintegratable.remove(&support[0]);
        }
        blocks_fallen.push(Block{x: block.x, y: block.y, z: (z, z+z1-z0)});
        disintegratable.insert(i);
    }

    println!("Disintegratable: {}", disintegratable.len());
}
