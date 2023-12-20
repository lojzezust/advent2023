use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, VecDeque};

type Signal = (String,String,bool);
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String,bool>),
}

impl Module {
    fn update(&mut self, signal: &Signal) -> Option<bool> {
        let (from, _, high) = signal;

        // update state
        match self {
            Module::FlipFlop(state) => {
                if !high {*state = !*state};
            },
            Module::Conjunction(states) => {
                states.insert(from.to_string(), *high);
            },
        };

        self.output(high)
    }

    fn output(&self, high: &bool) -> Option<bool>{
        match self {
            Module::FlipFlop(state) => if !high {Some(*state)} else {None},
            Module::Conjunction(states) => Some(!states.values().all(|x| *x))
        }
    }
}

fn main() {
    let f = File::open("inputs/day20.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse modules
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut in_connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut out_connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut start_modules: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let mut it = line.split(" -> ");
        let spec = it.next().expect("Missing spec");
        let connections: Vec<String> = it.next().expect("Missing connections").split(", ").map(String::from).collect();

        if spec == "broadcaster" {
            start_modules = connections;
            continue;
        }

        let t = spec.chars().nth(0).expect("Missing type");
        let name = &spec[1..];

        for conn in connections.iter() {
            out_connections
                .entry(name.to_string())
                .or_insert(Vec::new())
                .push(conn.to_string());

            in_connections
                .entry(conn.to_string())
                .or_insert(Vec::new())
                .push(name.to_string());
        }
        

        let module = match t {
            '%' => Module::FlipFlop(false),
            '&' => Module::Conjunction(HashMap::new()),
            _ => panic!("Unknown module type"),
        };
        modules.insert(String::from(name), module);
    }

    // Init state (for conjunctions)
    for (name, module) in modules.iter_mut() {
        match module {
            Module::Conjunction(ref mut states) => {
                for conn in in_connections.get(name).unwrap().iter() {
                    states.insert(conn.to_string(), false);
                }
            },
            _ => {},
        }
    }

    // Process button presses
    let mut num_pos: u64 = 0;
    let mut num_neg: u64 = 0;
    for _ in 0..1000 {
        num_neg += 1; // button press sends a negative signal
        let mut queue: VecDeque<Signal> = VecDeque::new();
        for name in start_modules.iter() {
            queue.push_back((String::from(""), name.to_string(), false));
        }

        while let Some(signal) = queue.pop_front() {
            let (_, to, high) = &signal;

            if *high {
                num_pos += 1;
            } else {
                num_neg += 1;
            }

            if let Some(module) = modules.get_mut(to){
                let out_pulse = module.update(&signal);
            
                if let Some(out_pulse) = out_pulse {
                    for conn in out_connections.get(to).unwrap().iter() {
                        queue.push_back((to.to_string(), conn.to_string(), out_pulse));
                    }
                }
            }
        }
    }

    println!("Pos: {}, Neg: {}", num_pos, num_neg);
    println!("Result: {}", num_pos * num_neg);
}

