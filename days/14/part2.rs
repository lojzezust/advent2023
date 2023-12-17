use std::collections::HashMap;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

type Matrix<T> = Vec<Vec<T>>;

#[derive(PartialEq)]
enum Tile {
    Rock,
    Obst,
    Empty
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Rock,
            '#' => Tile::Obst,
            _ => Tile::Empty
        }
    }

    fn to_char(tile: &Self) -> char {
        match tile {
            Tile::Rock => 'O',
            Tile::Obst => '#',
            Tile::Empty => '.'
        }
    }
}

/// Tilt north
fn tilt_n(pat: &mut Matrix<Tile>){
    let (n,m) = (pat.len(), pat[0].len());

    for col_i in 0..m {
        let mut li = 0;
        let mut ri = 1;
        while ri < n && li < n {
            if pat[li][col_i] != Tile::Empty {
                li += 1;
                continue;
            }
            if ri <= li {
                ri = li + 1;
                continue;
            }

            if pat[ri][col_i] == Tile::Rock {
                pat[li][col_i] = Tile::Rock;
                pat[ri][col_i] = Tile::Empty;
            } else if pat[ri][col_i] == Tile::Obst {
                li = ri;
            }
            ri += 1;
        }
    }

}

/// Tilt south
fn tilt_s(pat: &mut Matrix<Tile>){
    let (n, m) = (pat.len(), pat[0].len());

    for col_i in 0..m {
        let mut li: i32 = n as i32 - 1;
        let mut ri: i32 = n as i32 - 2;
        while ri >= 0 && li >= 0 {
            let (uli, uri) = (li as usize, ri as usize);
            if pat[uli][col_i] != Tile::Empty {
                li -= 1;
                continue;
            }
            if ri >= li {
                ri = li - 1;
                continue;
            }

            if pat[uri][col_i] == Tile::Rock {
                pat[uli][col_i] = Tile::Rock;
                pat[uri][col_i] = Tile::Empty;
            } else if pat[uri][col_i] == Tile::Obst {
                li = ri;
            }
            ri -= 1;
        }
    }

}

/// Tilt west
fn tilt_w(pat: &mut Matrix<Tile>){
    let (n, m) = (pat.len(), pat[0].len());

    for row_i in 0..n {
        let mut li = 0;
        let mut ri = 1;
        while ri < m && li < m{
            if pat[row_i][li] != Tile::Empty {
                li += 1;
                continue;
            }
            if ri <= li {
                ri = li + 1;
                continue;
            }

            if pat[row_i][ri] == Tile::Rock {
                pat[row_i][li] = Tile::Rock;
                pat[row_i][ri] = Tile::Empty;
            } else if pat[row_i][ri] == Tile::Obst {
                li = ri;
            }
            ri += 1;
        }
    }

}

/// Tilt east
fn tilt_e(pat: &mut Matrix<Tile>){
    let (n, m) = (pat.len(), pat[0].len());

    for row_i in 0..n {
        let mut li: i32 = m as i32 - 1;
        let mut ri: i32 = m as i32 - 2;
        while ri >= 0 && li >= 0 {
            let (uli, uri) = (li as usize, ri as usize);
            if pat[row_i][uli] != Tile::Empty {
                li -= 1;
                continue;
            }
            if ri >= li {
                ri = li - 1;
                continue;
            }

            if pat[row_i][uri] == Tile::Rock {
                pat[row_i][uli] = Tile::Rock;
                pat[row_i][uri] = Tile::Empty;
            } else if pat[row_i][uri] == Tile::Obst {
                li = ri;
            }
            ri -= 1;
        }
    }
}

fn tilt_cycle(pat: &mut Matrix<Tile>){
    tilt_n(pat);
    tilt_w(pat);
    tilt_s(pat);
    tilt_e(pat);
}

/// Converts matrix to string (for hashing)
fn pat_to_str(pat: &Matrix<Tile>) -> String{
    pat.iter()
        .flat_map(|row| row.iter().map(Tile::to_char))
        .collect()
}

/// Build matrix from string
fn str_to_pat(s: &str, n:usize, m:usize) -> Matrix<Tile> {
    let mut pat: Matrix<Tile> = Matrix::new();
    let mut it = s.chars();
    for _ in 0..n {
        let mut row: Vec<Tile> = Vec::new();
        for _ in 0..m {
            let t = Tile::from(it.next().unwrap());
            row.push(t);
        }
        pat.push(row);
    }

    pat
}

fn find_final(pat: &mut Matrix<Tile>, n_steps: usize) -> Matrix<Tile> {
    let (n,m) = (pat.len(), pat[0].len());

    let mut states: HashMap<String, usize> = HashMap::new();
    let mut states_arr: Vec<String> = Vec::new(); 
    let mut i = 0;
    let s = pat_to_str(&pat);
    states.insert(s.clone(), i);
    states_arr.push(s);
    let (istart, cycle_len) = loop {
        tilt_cycle(pat);
        i += 1;
        let s = pat_to_str(&pat);

        // If we reach the end before a cycle is found
        if i == n_steps {
            return str_to_pat(&s, n, m);
        }

        if states.contains_key(&s) {
            // Found cycle
            let istart = states[&s];
            break (istart, i - istart);
        }

        states.insert(s.clone(), i);
        states_arr.push(s);
    };

    println!("Cycle start: {}, length: {}", istart, cycle_len);

    // Compute final position in the cycle
    let final_i = istart + (n_steps - istart) % cycle_len;
    let final_s = &states_arr[final_i];

    str_to_pat(final_s, n, m)
}

fn main() {
    let f = File::open("inputs/day14.txt").expect("Missing file");
    let reader = BufReader::new(f);

    // Parse pattern
    let mut pat: Matrix<Tile> = Matrix::new();
    for line in reader.lines(){
        let line = line.expect("Error reading line.");
        
        if line.len() == 0 {
            break;
        }
        
        let tiles: Vec<Tile> = line.chars().map(Tile::from).collect();
        pat.push(tiles);
    }

    // Find final configuration
    let pat_final = find_final(&mut pat, 1_000_000_000);

    // Compute load
    let n = pat_final.len();
    let mut total = 0;
    for (i, row) in pat_final.iter().enumerate() {
        for el in row.iter() {
            total += match el {
                Tile::Rock => n - i,
                _ => 0
            };
        }
    }

    println!("Total: {}", total);
}
