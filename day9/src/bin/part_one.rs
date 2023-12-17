use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
struct History {
    sequence: Vec<i128>,
}

impl History {
    fn create_prediction_sequences(&self) -> Vec<Vec<i128>> {
        let mut res: Vec<Vec<i128>> = Vec::new();
        res.push(self.sequence.clone());
        while !is_all_zeros(res.last().unwrap()) {
            res.push(get_diff(res.last().unwrap()));
        } 
        return res
    }
}

fn main() {
    let file: File = File::open("./input.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);
    
    let mut histories: Vec<History> = Vec::new();
    
    for line in buf.lines() {
        histories.push(parse_line_to_history(line.unwrap()));
    }

    let mut prediction_values: Vec<i128> = Vec::new();
    for h in histories {
        let sequences = h.create_prediction_sequences();
        prediction_values.push(extrapolate_from_sequences(sequences));
    }
    println!("Result: {}", prediction_values.iter().sum::<i128>());
}

fn parse_line_to_history(l: String) -> History {
    let mut v: Vec<i128> = Vec::new();
    let split_str = l.split_ascii_whitespace().collect::<Vec<&str>>();
    for val in split_str {
        let curr_val = val.parse::<i128>().unwrap();
        v.push(curr_val);
    }
    return History { sequence: v }
}

fn get_diff(v: &Vec<i128>) -> Vec<i128> {
    let mut res: Vec<i128> = Vec::new();
    for i in 0..v.len()-1 {
        res.push(v[i+1] - v[i]);
    }
    return res;
}

fn is_all_zeros(v: &Vec<i128>) -> bool {
    return v.iter().filter(|x| **x != 0).count() == 0;
}

fn extrapolate_from_sequences(mut v: Vec<Vec<i128>>) -> i128 {
    let mut prev: i128;
    let mut curr: i128;
    
    let mut i = v.len() - 1;
    v[i].push(0);

    while i > 0 {
        prev = *v[i-1].last().unwrap();
        curr = *v[i].last().unwrap();
        v[i-1].push(curr+prev);
        i -= 1;
        
    }
    let res: i128 = *v[0].last().unwrap();
    return res;
}