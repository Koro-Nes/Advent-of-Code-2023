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
    let race = lines_to_race(input);
    println!("Race: {:?}", race);
    let err_margin = race.get_num_of_ways();
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
fn lines_to_race(v: Vec<String>) -> Race {
    let mut split_lines: Vec<Vec<&str>> = Vec::new();
    for l in &v {
        let split: Vec<&str> = l.split_ascii_whitespace().collect();
        split_lines.push(split);
    }
    let time_splice = &split_lines[0][1..split_lines[0].len()];
    let distance_splice = &split_lines[1][1..split_lines[1].len()];

    let time = time_splice.join("").parse::<u128>().unwrap();
    let distance = distance_splice.join("").parse::<u128>().unwrap();
    return Race { duration: time, record_distance: distance }
}

#[inline]
fn get_time(hold_duration: u128, distance: u128) -> u128 {
    let velocity = hold_duration;
    return hold_duration + (distance / velocity);
}