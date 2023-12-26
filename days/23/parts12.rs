use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, VecDeque, HashMap};

type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug, PartialEq)]
enum Dir {Up, Down, Left, Right}

#[derive(Debug)]
enum Tile {
    Forest,
    Path,
    Slope(Dir)
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '^' => Tile::Slope(Dir::Up),
            'v' => Tile::Slope(Dir::Down),
            '>' => Tile::Slope(Dir::Right),
            '<' => Tile::Slope(Dir::Left),
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn is_path(&self) -> bool {
        match self {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(_) => true,
        }
    }

    fn can_enter(&self, dir: Dir) -> bool {
        match self {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(slope_dir) => dir == *slope_dir,
        }
    }

    fn can_leave(&self, dir: Dir) -> bool {
        match self {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(slope_dir) => dir == *slope_dir,
        }
    }
}

struct Item {
    pos: (usize, usize),
    dist: usize,
    prev_node: usize
}

fn _longest_path(edges: &HashMap<usize, Vec<(usize, usize)>>, cur: usize, end:usize, dist: usize, visited: &mut HashSet<usize>) -> usize {
    if cur == end {
        return dist;
    }
    let mut max_dist = 0;
    if let Some(neighbours) = edges.get(&cur) {
        for (node, edge_dist) in neighbours {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(*node);
            let new_dist = _longest_path(edges, *node, end, dist + edge_dist, visited);
            if new_dist > max_dist {
                max_dist = new_dist;
            }
            visited.remove(&node);
        }
    }
    max_dist
}

fn longest_path(edges: &HashMap<usize, Vec<(usize, usize)>>, start: usize, end:usize) -> usize {
    let mut visited = HashSet::from([0]);
    _longest_path(edges, start, end, 0, &mut visited)
}

fn main() {
    let f = File::open("inputs/day23.txt").expect("Missing file");
    let reader = BufReader::new(f);
    
    // Parse map
    let map: Matrix<Tile> = reader.lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
        .collect();

    // Build a graph of the map
    let mut nodes = Vec::from([(0,1)]);
    let mut edges = HashMap::new();
    let mut edges_p2 = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([Item{pos: (0,1), dist: 0, prev_node: 0}]);
    let mut end_i = 0;
    while let Some(Item{pos:(i,j), dist, prev_node}) = queue.pop_front() {
        if visited.contains(&(i,j)) {
            continue;
        }
        // println!("{},{}, with dist {} and prev_node {}", i, j, dist, prev_node);
        
        let mut candidates = Vec::new();
        let mut paths = 0;
        if i > 0 && map[i-1][j].is_path() {
            paths += 1;
            if map[i][j].can_leave(Dir::Up) && map[i-1][j].can_enter(Dir::Up) && !visited.contains(&(i-1,j)) {
                candidates.push((i-1,j));
            }
        }
        if j > 0 && map[i][j-1].is_path() {
            paths += 1;
            if map[i][j].can_leave(Dir::Left) && map[i][j-1].can_enter(Dir::Left) && !visited.contains(&(i,j-1)) {
                candidates.push((i,j-1));
            }
        }
        if i < map.len()-1 && map[i+1][j].is_path() {
            paths += 1;
            if map[i][j].can_leave(Dir::Down) && map[i+1][j].can_enter(Dir::Down) && !visited.contains(&(i+1,j)) {
                candidates.push((i+1,j));
            }
        }
        if j < map[0].len()-1 && map[i][j+1].is_path() {
            paths += 1;
            if map[i][j].can_leave(Dir::Right) && map[i][j+1].can_enter(Dir::Right) && !visited.contains(&(i,j+1)) {
                candidates.push((i,j+1));
            }
        }

        let mut new_node = prev_node;
        let mut dist = dist;
        if let Some(node_i) = nodes.iter().position(|x| *x == (i,j)) {
            if node_i != prev_node {
                edges.entry(prev_node).or_insert(Vec::new()).push((node_i, dist));
                edges_p2.entry(prev_node).or_insert(Vec::new()).push((node_i, dist));
                edges_p2.entry(node_i).or_insert(Vec::new()).push((prev_node, dist));
            }
            new_node = node_i;
            dist = 0;
        } else if paths != 2 {
            let new_pos = (i,j);
            nodes.push(new_pos);
            new_node = nodes.len()-1;
            edges.entry(prev_node).or_insert(Vec::new()).push((new_node, dist));
            edges_p2.entry(prev_node).or_insert(Vec::new()).push((new_node, dist));
            edges_p2.entry(new_node).or_insert(Vec::new()).push((prev_node, dist));
            dist = 0;

            if i == map.len()-1 && j == map[0].len()-2{
                end_i = new_node;
            }
        }

        if new_node == prev_node {
            visited.insert((i,j));
        }

        for (i,j) in candidates {
            queue.push_back(Item{pos: (i,j), dist: dist+1, prev_node: new_node});
        }
    }

    // println!("Nodes: {:?}", nodes);
    // println!("Edges: {:?}", edges);
    println!("Nodes: {}", nodes.len());
    println!("Edges: {}", edges.values().map(|x| x.len()).sum::<usize>());

    let len = longest_path(&edges, 0, end_i);
    println!("Longest path (P1): {}", len);

    let len = longest_path(&edges_p2, 0, end_i);
    println!("Longest path (P2): {}", len);
}
