use std::{fs::File, io::BufReader};
use std::io::prelude::*;

type Matrix<T> = Vec<Vec<T>>;

fn parse_pattern(lines: &mut impl Iterator<Item = Result<String, std::io::Error>>) -> Matrix<char> {
    let mut pat: Matrix<char> = Matrix::new();
    while let Some(line) = lines.next(){
        let line = line.expect("Error reading line.");
        
        if line.len() == 0 {
            break;
        }
        
        let chars: Vec<char> = line.chars().collect();
        pat.push(chars);
    }

    pat
}

fn find_sym_v(pattern: &Matrix<char>, skip: Option<usize>) -> Option<usize> {
    // Finds first vertical symmetry line (ignores 'skip' if found)
    let mut cols = vec![true; pattern[0].len()-1];
    let skip = skip.unwrap_or(cols.len());
    for row in pattern{
        for start_i in 0..row.len()-1 {
            let l = std::cmp::min(start_i+1, row.len() - start_i - 1);
            let mirrored = (0..l).all(|j| row[start_i - j] == row[start_i + j + 1]);
            if !mirrored {
                cols[start_i] = false;
            }
        }
    }

    // Return number of columns to the left of first mirror (i+1) or None
    cols.iter().enumerate().find(|&(i,x)| *x && i != skip).map(|(i,_)| i + 1)
}


fn find_sym_h(pattern: &Matrix<char>, skip: Option<usize>) -> Option<usize> {
    // Finds first horizontal symmetry line (ignores 'skip' if found)
    let mut rows = vec![true; pattern.len()-1];
    let n_row = pattern.len();
    let n_col = pattern[0].len();
    let skip = skip.unwrap_or(n_row);
    for col_i in 0..n_col{
        for start_i in 0..n_row-1 {
            let l = std::cmp::min(start_i+1, n_row - start_i - 1);
            let mirrored = (0..l).all(|j| pattern[start_i - j][col_i] == pattern[start_i + j + 1][col_i]);
            if !mirrored {
                rows[start_i] = false;
            }
        }
    }

    // Return number of rows above first mirror (i+1) or None
    rows.iter().enumerate().find(|&(i,x)| *x && i != skip).map(|(i,_)| i + 1)
}

fn invert(c: char) -> char {
    match c {
        '.' => '#',
        '#' => '.',
        _ => panic!("Invalid character!")
    }
}

fn find_smudge(pattern: &mut Matrix<char>) -> (Option<usize>, Option<usize>){
    let exh = find_sym_h(pattern, None);
    let exv = find_sym_v(pattern, None);

    // Invert every character until you find new symmetry
    for i in 0..pattern.len(){
        for j in 0..pattern[i].len(){
            pattern[i][j] = invert(pattern[i][j]);
            let h = find_sym_h(pattern, exh.map(|x| x-1));
            let v = find_sym_v(pattern, exv.map(|x| x-1));
            pattern[i][j] = invert(pattern[i][j]);

            if h.is_some() || v.is_some() {
                return (h, v);
            }
        }
    }

    (None, None)
}

fn main() {
    let f = File::open("inputs/day13.txt").expect("Missing file");
    let reader = BufReader::new(f);

    let mut lines = reader.lines().peekable();

    let mut total = 0;
    while lines.peek().is_some() {
        let mut pattern = parse_pattern(&mut lines);
        
        let (h,v) = find_smudge(&mut pattern);
        // println!("{:?}, {:?}", h, v);
        total = match (h,v) {
            (_, Some(cols)) => total + cols,
            (Some(rows), _) => total + 100 * rows,
            _ => total
        };
    }

    println!("Total: {}", total);
}
