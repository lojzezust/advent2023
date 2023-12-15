use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn get_hash(string: &str) -> usize {
    let mut hash = 0;
    for c in string.chars() {
        let ascii = c as i32;
        hash = ((hash + ascii) * 17) % 256;
    }
    hash as usize
}

fn remove(box_i: &mut Vec<(String,i32)>, lbl: &str) {
    if let Some((i, _)) = box_i.iter().enumerate().find(|(_,(l,_))| l == lbl) {
        box_i.remove(i);
    }
}

fn insert(box_i: &mut Vec<(String,i32)>, lbl: &str, num: i32) {
    if let Some((i, _)) = box_i.iter().enumerate().find(|(_,(l,_))| l == lbl) {
        box_i[i] = (String::from(lbl), num);
    } else {
        box_i.push((String::from(lbl), num));
    };
}

fn main() {
    let f = File::open("inputs/day15.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let line = reader.lines().next().unwrap().expect("Error reading line");

    let mut boxes: Vec<Vec<(String,i32)>> = vec![Vec::new(); 256];
    for cmd in line.split(',') {
        if cmd.contains('-'){
            let lbl = &cmd[..cmd.len()-1];
            let hash = get_hash(lbl);
            remove(&mut boxes[hash], lbl);
        } else {
            let mut s = cmd.split('=');
            let lbl = s.next().unwrap();
            let n: i32 = s.next().unwrap().parse().expect("Error parsing integer");
            
            let hash = get_hash(lbl);
            insert(&mut boxes[hash], lbl, n);
        }
        // println!("{}: {}", cmd, hash(cmd));
    }

    let mut total = 0;
    for (i,abox) in boxes.iter().enumerate() {
        for (j,(_, n)) in abox.iter().enumerate() {
            total += (1+i as i32) * (1+j as i32) * n;
        }
    }
    println!("Total focusing power: {}", total);
}
