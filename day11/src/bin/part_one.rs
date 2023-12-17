use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

#[derive(Debug)]
struct Galaxies {
    galaxies: HashMap<usize, Position>,
}

impl Galaxies {
    fn from_map(v: HashMap<usize, Position>) -> Galaxies {
        let mut distances: HashMap<(Position, Position), usize> = HashMap::new();
        let count = v.len();
        let mut curr_id = 0;
        for p in &v {
            for i in 0..count {
                if i == curr_id {
                    continue;
                }
                let other = v[&i].clone();
                let distance = p.1.distance_to(&other);
                if !distances.contains_key(&(p.1.clone(), other.clone())) {
                    distances.insert((p.1.clone(), other.clone()), distance);
                }
                if !distances.contains_key(&(other.clone(), p.1.clone())) {
                    distances.insert((other.clone(), p.1.clone()), distance);
                }
            }
            curr_id += 1;
        }

        return Galaxies {  galaxies: v };
    }
    fn into_distance_pairs(&self) -> HashMap<(usize, usize), usize> {
        let mut hm: HashMap<(usize, usize), usize> = HashMap::new();
        for id in 0..self.galaxies.len() {
            for other in id+1..self.galaxies.len() {
                if !hm.contains_key(&(id, other)) && 
                    !hm.contains_key(&(other, id)) 
                    {
                        hm.insert((id, other), 
                            self.galaxies.get(&id)
                            .unwrap()
                            .distance_to(&self.galaxies
                                .get(&other)
                                .unwrap()));
                    }
            }
        }

        hm
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn distance_to(&self, other: &Position) -> usize {
    
        let sx: i32 = self.x as i32;
        let sy: i32 = self.y as i32;
        let ox: i32 = other.x as i32;
        let oy: i32 = other.y as i32;
        let distx = if sx > ox {
            sx - ox
        } else {
            ox - sx
        };
        let disty = if sy > oy {
            sy - oy
        } else {
            oy - sy
        };
        return (distx + disty) as usize;
    }
}

fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);
    let grid = parse_input(buf);
    let expand_grid = expand_empty_space(grid);
    let positions = parse_to_positions(expand_grid);
    let mut hm: HashMap<usize, Position> = HashMap::new();
    let mut id = 0;
    positions.iter().for_each(|x| { hm.insert(id, x.clone()); id += 1 });
    let galaxies = Galaxies::from_map(hm);
    let distance_pairs = galaxies.into_distance_pairs();
    let distance_sum = distance_pairs
                                .into_iter()
                                .map(|x| x.1)
                                .collect::<Vec<usize>>()
                                .iter()
                                .sum::<usize>();
    println!("Result: {}", distance_sum);
}

fn parse_input(buf: BufReader<File>) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = Vec::new();
    for l in buf.lines() {
        let mut curr_line = Vec::new();
        let s = l.unwrap();
        for c in s.chars() {
            curr_line.push(c); 
       }
       v.push(curr_line);
    }
    v
}

fn expand_empty_space(g: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = g.clone();
    let mut hs_col: HashSet<usize> = HashSet::new();
    let mut hs_row: HashSet<usize> = HashSet::new();

    for i in 0..g.len() {
        if !hs_col.contains(&i) {
            if !g[i].contains(&'#') {
                hs_col.insert(i);
            }
        }
    for j in 0..g[i].len() {
        if !hs_row.contains(&j) {
            if is_in_col(&g, j) {
                hs_row.insert(j);
            }        
        }
    }
    }
    let len = g[0].len();
    let mut c_counter = 0;
    let mut col_iter = hs_col.into_iter().collect::<Vec<usize>>();
    col_iter.sort();
    for c in col_iter {
        let next = get_empty_row(len);
        v.insert(c+c_counter, next);
        c_counter += 1;
    }
    let mut r_counter = 0;
    let mut row_iter = hs_row.into_iter().collect::<Vec<usize>>();
    row_iter.sort();
    for r in row_iter {
        for n in 0..v.len() {
            v[n].insert(r+r_counter, '.');
        }
        r_counter += 1;
    }

    v
}

fn get_empty_row(len: usize) -> Vec<char>{
    let mut v = Vec::new();
    for _i in 0..len {
        v.push('.');
    }
    v
}

fn is_in_col(v: &Vec<Vec<char>>, i: usize) -> bool {
    for n in 0..v[i].len() {
        if v[n][i] == '#' {
            return false;
        }
    }
    return true;
}

fn parse_to_positions(g: Vec<Vec<char>>) -> Vec<Position> {
    let mut v = Vec::new();
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == '#' {
                v.push(Position { x: j, y: i });
            }
        }
    }
    v
}