use std::{fs::File, io::BufReader};
use std::collections::HashMap;
use std::io::prelude::*;
use std::cmp;

enum Conclusion {
    Accept,
    Reject,
    Next(String)
}

impl Conclusion {
    fn from_str(s: &str) -> Conclusion {
        match s {
            "A" => Conclusion::Accept,
            "R" => Conclusion::Reject,
            _ => Conclusion::Next(s.to_string())
        }
    }
}

struct Workflow {
    conditions: Vec<(char, cmp::Ordering, i32, Conclusion)>,
    default: Conclusion
}

#[derive(Debug)]
struct Part(Vec<i32>);

impl Part {
    fn get(&self, c: char) -> i32 {
        match c {
            'x' => self.0[0],
            'm' => self.0[1],
            'a' => self.0[2],
            's' => self.0[3],
            _ => panic!("Invalid variable")
        }
    }

    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}

fn main() {
    let f = File::open("inputs/day19.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse workflows
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.expect("Error reading line");
        if line == "" {
            break;
        }

        let mut s = line.split('{');
        let name = s.next().unwrap();
        let rem = s.next().unwrap();

        let mut conditions = Vec::new();
        let mut default = Conclusion::Reject;
        for conds in rem[..rem.len()-1].split(',') {
            if !conds.contains(':') {
                default = Conclusion::from_str(conds);
                break;
            }
            let mut s = conds.split(':');
            let cond = s.next().unwrap();
            let conclusion = Conclusion::from_str(s.next().unwrap());
            let mut it = cond.chars();
            let var = it.next().unwrap();
            let op = it.next().unwrap();
            let cmp = match op {
                '<' => cmp::Ordering::Less,
                '>' => cmp::Ordering::Greater,
                _ => panic!("Invalid comparison")
            };
            let val: i32 = cond[2..].parse().unwrap();

            conditions.push((var, cmp, val, conclusion));
        }
        let workflow = Workflow { conditions, default };
        workflows.insert(name.to_string(), workflow);
    }

    // Parse parts
    let mut parts = Vec::new();
    while let Some(line) = it.next() {
        let line = line.expect("Error reading line");
        let line = &line[1..line.len()-1];
        let vals: Vec<i32> = line.split(',')
            .map(|s| *(&s[2..].parse::<i32>().unwrap()))
            .collect();
        parts.push(Part(vals));
    }

    // Run workflows
    let mut total = 0;
    for part in parts {
        let mut w_name = String::from("in");
        loop {
            let workflow = workflows.get(&w_name).unwrap();
            let mut conclusion = &workflow.default;
            for (var, cmp, val, conc) in &workflow.conditions {
                let part_val = part.get(*var);
                if part_val.cmp(val) == *cmp {
                    conclusion = &conc;
                    break;
                }
            }
            match conclusion {
                Conclusion::Accept => {
                    // println!("Accept: {}: {:?}", w_name, part);
                    total += part.sum();
                    break;
                },
                Conclusion::Reject => {
                    break;
                },
                Conclusion::Next(name) => {
                    w_name = name.to_string();
                }
            }
        }
    }

    println!("Total: {}", total);
}

