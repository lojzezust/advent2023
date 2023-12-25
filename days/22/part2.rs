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
    
    // Parse blocks
    let mut blocks = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let block = Block::from_str(&line);
        blocks.push(block);
    }

    // Sort by z
    blocks.sort_by(|a,b| a.z.cmp(&b.z));

    let mut blocks_fallen = Vec::new();
    let mut would_fall: Vec<HashSet<usize>> = Vec::new();
    for (i,block) in blocks.iter().enumerate() {
        let Block{x: _, y: _, z: (z0,z1)} = block;
        let (z, support) = find_fall_place(&blocks_fallen, &block);

        if support.len() > 0 {
            // Add block as fallen to all blocks which remove the entire support
            for rem in would_fall.iter_mut() {
                if support.iter().all(|x| rem.contains(x)) {
                    rem.insert(i);
                }
            }
        }

        blocks_fallen.push(Block{x: block.x, y: block.y, z: (z, z+z1-z0)});
        would_fall.push(HashSet::from([i]));
    }

    // Sum up all blocks which would fall (-1 because the block itself is not counted)
    let res: usize = would_fall.iter().map(|x| x.len() - 1).sum();
    println!("Fallen bricks: {}", res);
}
