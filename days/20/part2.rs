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

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    let f = File::open("inputs/day20.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse modules
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut in_connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut out_connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut start_modules: Vec<String> = Vec::new();
    let mut out_module: Option<String> = None;
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
            if conn == "rx" {
                out_module = Some(name.to_string());
            }

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

    // Find the cycle lengths for all module connecting into out_module
    let mut freq: HashMap<String, i64> = HashMap::new();
    'outer: for i in 1..100_000 {
        let mut queue: VecDeque<Signal> = VecDeque::new();
        for name in start_modules.iter() {
            queue.push_back((String::from(""), name.to_string(), false));
        }

        while let Some(signal) = queue.pop_front() {
            let (from, to, high) = &signal;

            if *high && out_module == Some(to.to_string()) {
                freq.entry(from.to_string()).or_insert(i);
                if freq.len() == in_connections.get(out_module.as_ref().unwrap()).unwrap().len() {
                    break 'outer;
                }
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

    let mut total: i64 = 1;
    // When will all the modules send 'high' at the same time? LCM
    for (k,v) in freq.iter() {
        println!("{} sends -high-> pulse every {} presses", k, v);
        total = lcm(total, *v);
    }
    println!("Result: {}", total);
}

