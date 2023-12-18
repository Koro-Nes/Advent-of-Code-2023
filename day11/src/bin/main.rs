use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, time::Instant};

    /*****************************/
    /* Both parts can be done at */
    /* with the functions by     */
    /* changing empty_offset in  */
    /* calculate_distances (L22) */
    /*****************************/


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i128,
    y: i128,
    offset_x: i128,
    offset_y: i128,
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);
    let empty_offset = 999_999; // 1 => part one, 999_999 => part two
    let now = Instant::now();
    let hm = parse_to_positions(buf);
    let cols = find_empty_cols(&hm);
    let rows = find_empty_rows(&hm);
    let pairs = get_pairs(&hm);
    let distances = calculate_distances(pairs, empty_offset, rows, cols);
    let sum = distances.iter().sum::<i128>();
    println!("Sum: {}", sum);
    println!("Finished in: {:?}", now.elapsed());
}

fn parse_to_positions(buf: BufReader<File>) -> HashMap<usize, Position> {
    let mut hm: HashMap<usize, Position> = HashMap::new();
    let mut curr_id = 0;

    let mut x = 0;
    let mut y;
    for l in buf.lines() {
        let curr_l = l.unwrap();
        y = 0;
        for c in curr_l.chars() {
            if c == '#' {
                hm.insert(curr_id, Position { x, y, offset_x: 0, offset_y: 0});
                curr_id += 1;
            }
            y += 1;
        }
        x += 1;
    }
    hm
}

fn find_empty_cols(hm: &HashMap<usize, Position>) -> Vec<i128> {
    let values = hm.values().map(|x| x.x).collect::<Vec<i128>>();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    let mut res = Vec::new();
    for i in *min..*max+1 {
        if !values.contains(&i) {
            res.push(i);
        }
    }
    res
}

fn find_empty_rows(hm: &HashMap<usize, Position>) -> Vec<i128> {
    let values = hm.values().map(|x| x.y).collect::<Vec<i128>>();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    let mut res = Vec::new();

    for i in *min..*max+1 {
        if !values.contains(&i) {
            res.push(i);
        }
    }    

    res
}

fn get_pairs(hm: &HashMap<usize, Position>) -> Vec<(Position, Position)> {
    let mut res = Vec::new();
    for i in 0..hm.keys().len() {
        for j in i+1..hm.keys().len() {
            let pair = (*hm.get(&i).unwrap(), *hm.get(&j).unwrap());
            res.push(pair);
        }
    }
    res
}

fn expand_pairs(v: Vec<(Position, Position)>, rows: Vec<i128>, cols: Vec<i128>, offset: i128) -> Vec<(Position, Position)> {
    let mut res: Vec<(Position, Position)> = Vec::new();
    for p in v {

        let mut p1 = p.0;
        let mut p2 = p.1;

        for c in &cols {
            if p1.x > *c + p1.offset_x {
                p1.x += offset;
                p1.offset_x += offset;
            }
            if p2.x > *c + p2.offset_x {
                p2.x += offset;
                p2.offset_x += offset
            }
        }
        for r in &rows {
            if p1.y > *r + p1.offset_y {
                p1.y += offset;
                p1.offset_y += offset
            }
            if p2.y > *r + p2.offset_y {
                p2.y += offset;
                p2.offset_y += offset
            }
        }
        res.push((p1, p2));
    }
    res
}


fn calculate_distances(pairs: Vec<(Position, Position)>, empty_offset: i128,
                      rows: Vec<i128>, cols: Vec<i128>) -> Vec<i128> {

    let mut res: Vec<i128> = Vec::new();
    let pairs = expand_pairs(pairs, rows, cols, empty_offset);
    for (p1, p2) in pairs {
        let distance = p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y);
        res.push(distance.try_into().unwrap());
        if p1.x == 0 && p1.y == 4 && p2.x == 10 && p2.y ==  9 {
            println!("{}", distance);
        }
    }
    res
}