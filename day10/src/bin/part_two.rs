use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    fn new(v: Vec<Vertex>) -> Graph {
        return Graph { vertices: v }
    }
    fn get_hashmap(&self) -> HashMap<(usize, usize), Vec<Edge>> {
        let mut hm: HashMap<(usize, usize), Vec<Edge>> = HashMap::new();

        for v in &self.vertices {
            hm.insert(v.pos, v.edges.clone());
        }

        return hm
    }
    fn get_edges(&self) {
        for e in &self.vertices {
            if !e.edges.is_empty() {
                println!("Vertex {}, {}: {:?}", e.pos.0, e.pos.1, e.edges)
            }
        }
    }
    fn traverse(&self, root: (usize, usize)) -> (Vec<(usize, usize)>, i32) {
        let v_map = self.get_hashmap();
        let start_edges = v_map.get(&root).unwrap();
        let mut queue: Vec<Edge> = Vec::new();
        let mut count = 0;

        for e in start_edges {
            queue.push(*e);
        }

        let mut visited: Vec<Edge> = Vec::new();
        let mut history: Vec<(usize, usize)> = Vec::new();

        while let Some(next) = queue.pop() {
            if contains_edge(&visited, &next) {
                continue;
            }

            if !history.contains(&next.source) {
                history.push(next.source);
            }

            visited.push(next);

            let n_edges = v_map.get(&next.dest).unwrap();
            for e in n_edges {
                if !contains_edge(&visited, e) {
                    queue.push(*e);
                }
            }
            count += 1;
        }
        return (history, count);
    }

}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Vertex {
    pos: (usize, usize),
    edges: Vec<Edge>
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Edge {
    source: (usize, usize),
    dest: (usize, usize),
}


impl Edge {
    // always sorts ascendingly
    fn new(x: (usize, usize), y: (usize, usize)) -> Edge {
        if x.0 < y.0 {
            return Edge { source: x, dest: y };
        } else if x.0 > y.0 {
            return Edge { source: y, dest: x };
        } else {
            if x.1 < y.1 {
                return Edge { source: x, dest: y };
            } else {
                return Edge { source: y, dest: x };
            }
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.source == other.source && self.dest == other.dest) || (self.source == other.dest && self.dest == other.source)
    }
}


fn main() {
    let file: File = File::open("./test1b.txt").expect("Invalid Filepath.");
    let buf: BufReader<File> = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();

    for l in buf.lines() {
        input.push(l.unwrap());
    }
    let graph_data = parse_input(&input);
    let graph = Graph::new(graph_data.0);
    let result = graph.traverse(graph_data.1);

    let s = parse_input(&input);
    let points = s.0.iter().map(|x| x.pos).collect::<Vec<(usize, usize)>>();
    get_enclosed_area(points, result.1);
    println!("Start: {:?}, {:?}", graph_data.1, result.0);
}

fn parse_input(v: &Vec<String>) -> (Vec<Vertex>, (usize, usize)) {
    let mut vertices: Vec<Vertex> = Vec::new();

    let mut i = 0;
    let mut j;
    let hm = get_dir_map();
    let mut start = (0,0);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in v {
        grid.push(line.chars().collect());
    }

    while i < grid.len() {
        j = 0;
        while j < grid[i].len() {
            let mut edges: Vec<Edge> = Vec::new();
            if grid[i][j] == 'S' {
                start = (i,j);
            }
            if grid[i][j] != '.' {

                let dir = *hm.get(&grid[i][j]).unwrap();
                if dir.contains("n") && i > 0 {
                    match grid[i - 1][j] {
                        '.' => (),
                        _ => {
                            let dir_dest = *hm.get(&grid[i - 1][j]).unwrap();
                            if dir_dest.contains("s") {
                                add_ndup(&mut edges, Edge::new((i,j), (i-1,j)), (i,j))
                            }
                        }
                    }
                }
                if dir.contains("s") && i + 1 < grid.len() {
                    match grid[i + 1][j] {
                        '.' => (),
                        _ => {
                            let dir_dest = *hm.get(&grid[i + 1][j]).unwrap();
                            if dir_dest.contains("n") {
                                add_ndup(&mut edges,Edge::new((i,j), (i+1,j)), (i,j))
                           }
                        }
                    }
                }
                if dir.contains("w") && j > 0 {
                    match grid[i][j - 1] {
                        '.' => (),
                        _ => {
                            let dir_dest = *hm.get(&grid[i][j - 1]).unwrap();
                            if dir_dest.contains("e") {
                                add_ndup(&mut edges,Edge::new((i,j), (i,j-1)), (i,j)) 
                            }
                        }
                    }
                }
                if dir.contains("e") && j + 1 < grid[i].len() {
                    match grid[i][j + 1] {
                        '.' => (),
                        _ => {
                            let dir_dest = *hm.get(&grid[i][j + 1]).unwrap();
                            if dir_dest.contains("w") {
                                add_ndup(&mut edges,Edge::new((i,j), (i,j+1)), (i,j)) 
                            }
                        }
                    }
                }
            }
            if !edges.is_empty() {
                vertices.push(Vertex { pos: (i, j), edges });
            }
            j += 1;
        }
        i += 1;
    }

    return (vertices, start);
}

fn get_dir_map() -> HashMap<char, &'static str> {
    let mut hm: HashMap<char, &str> = HashMap::new();

    hm.insert('|', "ns");
    hm.insert('-', "ew");
    hm.insert('L', "ne");
    hm.insert('J', "nw");
    hm.insert('7', "ws");
    hm.insert('F', "se");
    hm.insert('S', "nswe");

    return hm;
}

fn add_ndup(v: &mut Vec<Edge>, a: Edge, pos: (usize, usize)) {
    let mut b = false;

    let corrected_edge = if a.source == pos {
        a
    } else {
        Edge { source: a.dest, dest: a.source }
    }; 


    for e in v.clone() {
        if e == a {
         b = true;   
        }
    }
    if !b {
        v.push(corrected_edge);
    }
}

fn contains_edge(v: &Vec<Edge>, e: &Edge) -> bool {
    for x in v {
        if *x == *e {
            return true;
        }
    }
    return false;
}

fn get_minmax_coords(v: &Vec<(usize, usize)>) -> ((usize, usize),(usize, usize)) {
    let mut col_min = v[0].0;
    let mut col_max = v[0].0;
    let mut row_max = v[0].1;
    let mut row_min = v[0].1;

    for pos in v {
        if pos.0 < col_min {
            col_min = pos.0;
        }
        if pos.0 > col_max {
            col_max = pos.0;
        }
        if pos.1 < row_min {
            row_min = pos.1;
        }
        if pos.1 > row_max {
            row_max = pos.1;
        }
    }
    return ((col_min, row_min), (col_max, row_max));
}

fn create_enclosed_grid(v: &Vec<(usize, usize)>, min: (usize, usize), max: (usize, usize)) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for i in min.0..max.0+1 {
        let mut line: Vec<char> = Vec::new();
        for j in min.1..max.1+1 {
            if v.contains(&(i, j)) {
                line.push('X');
            } else {
                line.push('.');
            }
        }
        grid.push(line);
    }

    return grid;
}

fn get_enclosed_area(history: Vec<(usize, usize)>, pathlen: i32) {

    /* 
        Shoelace formula 
        A = (1/2) * Sum(y_i*(x_i-1-x_i+1))
    */ 

    let points = history_to_points(history);
    let sum: i32 = points
        .windows(2)
        .map(|x| { x[0].1 as i32 * x[1].0 as i32 - x[0].0 as i32 * x[1].1 as i32 })
        .sum();
    let area = sum.abs() / 2;
    let interior = (area + 1 - (points.len() as i32 / 2)).abs();
    println!("Result: {interior}");

}

fn history_to_points(v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    res
}