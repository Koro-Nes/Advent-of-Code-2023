use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

#[allow(dead_code)]

#[derive(Debug, Clone)]
struct Node<'a> {
    adj: Vec<(usize, usize)>,
    dir: &'a str,
}
#[allow(dead_code)]
impl Node<'_> {
    fn new(dir: &str) -> Node {
        let adj: Vec<(usize, usize)> = Vec::new();
        return Node { adj, dir };
    }
    fn new_with_adj<'a>(dir: &'a str, adjacent: Vec<(usize, usize)>) -> Node<'a> {
        return Node { adj: adjacent, dir };
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(hm: HashMap<(usize, usize), Node<'static>>) -> Graph {
        let mut edges: Vec<Edge> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();

        for (pos, node) in hm {
            vertices.push(Vertex { col: pos.0, row: pos.1 });
            for e in node.adj {
                let curr_edge = Edge { pos0: pos, pos1: e };
                if !check_if_already_included(&curr_edge, &edges) {
                    edges.push(curr_edge);
                }
            }
        }
        return Graph { vertices, edges };
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct Vertex {
    col: usize,
    row: usize,
}

#[derive(Debug)]
struct Edge {
    pos0: (usize, usize),
    pos1: (usize, usize),
}

impl Edge {
    fn is_equal(&self, other: &Edge) -> bool {
        return (self.pos0 == other.pos0 && self.pos1 == other.pos1) || (self.pos0 == other.pos1 && self.pos1 == other.pos0);
    }
}


fn main() {

    let file: File = File::open("./square.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);
    let dir_map: HashMap<char, &str> = get_dir_hashmap();
    let node_map: HashMap<(usize, usize), Node<'_>> = create_node_map(buf, &dir_map);
    let connected_graph: HashMap<(usize, usize), Node<'_>> = check_edges(node_map); 
    let graph = Graph::new(connected_graph);
    println!("{:?}", graph);
}

fn get_dir_hashmap() -> HashMap<char, &'static str> {
    let mut hm: HashMap<char, &'static str> = HashMap::new();

    hm.insert('|', "ns");
    hm.insert('-', "ew");
    hm.insert('L', "ne");
    hm.insert('J', "sw");
    hm.insert('7', "sw");
    hm.insert('F', "se");
    hm.insert('S', "nesw");


    return hm;
}

fn create_node_map(buf: BufReader<File>, dir_map: &HashMap<char, &'static str>) -> HashMap<(usize, usize), Node<'static>> {
    let mut hm: HashMap<(usize, usize), Node> = HashMap::new();
    let mut col: usize = 0;
    for line in buf.lines() {
        let mut row: usize = 0;
        let chars = line.unwrap().chars().collect::<Vec<char>>();
        for c in chars {
            if c != '.' {
                let dir = *dir_map.get(&c).unwrap();
                let curr_location = (col, row);
                let curr_node = Node::new(dir);
                hm.insert(curr_location, curr_node);
            }
            row += 1;
        }
        col += 1;
    }
    return hm;
}

fn check_edges(hm: HashMap<(usize, usize), Node<'static>>) -> HashMap<(usize, usize), Node<'static>> {

    let mut hm_clone: HashMap<(usize, usize), Node<'_>> = HashMap::new(); // needs to be cloned to statisfy borrow checker

    for node in &hm {
        let mut adjacent: Vec<(usize, usize)> = Vec::new();
        let node_dir = node.1.dir;
        let col = node.0.0;
        let row = node.0.1;

        if node_dir.contains("n") && col > 0 {
            if hm.contains_key(&(col-1, row)) {
                adjacent.push((col-1, row));
            }
        }
        if node_dir.contains("e") {
            if hm.contains_key(&(col, row+1)) {
                adjacent.push((col, row+1));
            }
        }
        if node_dir.contains("s") {
            if hm.contains_key(&(col+1, row)) {
                adjacent.push((col+1, row));
            }
        }    
        if node_dir.contains("w") && row > 0 {
            if hm.contains_key(&(col, row-1)) {
                adjacent.push((col, row-1));
            }
        }    
        hm_clone.insert((col, row), Node { adj: adjacent, dir: node_dir });
    }
    return hm_clone;
}

fn check_if_already_included(e: &Edge, v: &Vec<Edge>) -> bool {
    for a in v {
        if e.is_equal(&a) {
            return true;
        }
    }
    return false;
}
