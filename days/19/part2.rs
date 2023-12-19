use std::{fs::File, io::BufReader};
use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range(i32,i32);

impl Range {
    /// Split the range into two parts, one that satisfies the condition and one that doesn't
    fn process_cond(&self, cmp: cmp::Ordering, val: i32) -> (Range, Range) {
        let Range(mn,mx) = self;
        match cmp {
            cmp::Ordering::Less => {
                (Range(*mn, val-1), Range(val, *mx))
            },
            cmp::Ordering::Greater => {
                (Range(val+1, *mx), Range(*mn, val))
            },
            _ => panic!("Invalid comparison")
        }
    }
}

#[derive(Debug, Clone)]
struct Part(Vec<Range>);

impl Part {
    fn get(&self, c: char) -> Range {
        match c {
            'x' => self.0[0],
            'm' => self.0[1],
            'a' => self.0[2],
            's' => self.0[3],
            _ => panic!("Invalid variable")
        }
    }
    /// Update the range for the given variable
    fn set(&mut self, c: char, r: Range) {
        match c {
            'x' => self.0[0] = r,
            'm' => self.0[1] = r,
            'a' => self.0[2] = r,
            's' => self.0[3] = r,
            _ => panic!("Invalid variable")
        }
    }
    /// Creates a new part with the given range for the given variable
    fn with(&self, c: char, r: Range) -> Part {
        let mut new_ranges = self.0.clone();
        match c {
            'x' => new_ranges[0] = r,
            'm' => new_ranges[1] = r,
            'a' => new_ranges[2] = r,
            's' => new_ranges[3] = r,
            _ => panic!("Invalid variable")
        }
        Part(new_ranges)
    }
    /// Compute the number of possible combinations
    fn num(&self) -> i64 {
        let mut num = 1;
        for r in &self.0 {
            num = num * (r.1 - r.0 + 1) as i64;
            if r.1 < r.0 {
                println!("Invalid range: {:?}", r);
            }
        }
        num
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

    let mut queue = VecDeque::new();
    queue.push_back((String::from("in"), Part(vec![Range(1,4000);4])));
    // Run workflows
    let mut total = 0;
    while !queue.is_empty() {
        let (w_name, part) = queue.pop_front().unwrap();
        
        let workflow = workflows.get(&w_name).unwrap();
        let default = &workflow.default;
        let mut rem_part = part.clone();
        for (var, cmp, val, conc) in &workflow.conditions {
            // Split range into ok and nok parts
            let (range_ok, range_nok) = rem_part.get(*var).process_cond(*cmp, *val);
            rem_part.set(*var, range_nok);
            let ok_part = rem_part.with(*var, range_ok);
            match conc {
                Conclusion::Accept => {
                    // println!("Accept: {}: {:?}, {}", w_name, part, part.num());
                    total += ok_part.num();
                },
                Conclusion::Next(name) => {
                    queue.push_back((name.to_string(), ok_part));
                },
                _ => ()
            }
        }
        match default {
            Conclusion::Accept => {
                // println!("Accept: {}: {:?}, {}", w_name, rem_part, part.num());
                total += rem_part.num();
            },
            Conclusion::Next(name) => {
                queue.push_back((name.to_string(), rem_part));
            },
            _ => ()
        }
    }

    println!("Total: {}", total);
}
