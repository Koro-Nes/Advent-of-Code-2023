use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug)]
struct Race {
    duration: u128,
    record_distance: u128,
}

impl Race {
    #[inline]
    fn get_num_of_ways(&self) -> u128 {
        let mut count: u128 = 0;
        for i in 1..self.duration {
            if get_time(i, self.record_distance) < self.duration {
                count += 1;
            }
        }
        return count;
    }
}

fn main() {
    let file: File = File::open("./input.txt").unwrap();

    let buf = BufReader::new(file);
    let input = parse_lines(buf);
    let races = lines_to_races(input);
    let mut num_of_ways: Vec<u128> = Vec::new();
    for race in races {
        let ways = race.get_num_of_ways();
        num_of_ways.push(ways);
    }
    let err_margin = num_of_ways.iter().fold(1, |res, a| res * a);
    println!("Error Margin: {}", err_margin);
}

#[inline]
fn parse_lines(buf: BufReader<File>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for l in buf.lines() {
        res.push(l.unwrap().to_string());        
    }
    return res;
}

#[inline]
fn lines_to_races(v: Vec<String>) -> Vec<Race> {
    let mut res: Vec<Race> = Vec::new();
    let mut split_lines: Vec<Vec<&str>> = Vec::new();
    for l in &v {
        let split: Vec<&str> = l.split_ascii_whitespace().collect();
        split_lines.push(split);
    }

    for i in 1..split_lines[0].len() {
        res.push(Race { duration: split_lines[0][i].parse::<u128>().unwrap(),
             record_distance: split_lines[1][i].parse::<u128>().unwrap() });
    }

    return res;
}

#[inline]
fn get_time(hold_duration: u128, distance: u128) -> u128 {
    let velocity = hold_duration;
    return hold_duration + (distance / velocity);
}