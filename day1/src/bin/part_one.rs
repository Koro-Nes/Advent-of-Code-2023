use std::{fs::File, io::{BufReader, BufRead}};

fn main() {

    let file = File::open("./input.txt").expect("Invalid input path");
    let buf_reader = BufReader::new(file);

    let mut result_vec: Vec<u32> = Vec::new();

    for line in buf_reader.lines() {
        let line_num_tuple = get_num(line.unwrap().trim().to_string());
        let line_num = append_nums(line_num_tuple);
        result_vec.push(line_num);
    }
    let sum: u32 = result_vec.iter().sum();
    print!("{}", sum);
}

fn get_num(str: String) -> (u32, u32) {

    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut eflag: bool = false;
    let mut iter = str.chars();
    let mut start_index: usize = 0;
    let mut end_index: usize = iter.clone().count() - 1;
    let mut c: char;

    while start_index <= end_index {
        c = iter.next().expect("Line 27");
        if c.is_numeric() {
            start = c.to_digit(10).expect("Line 30");
            break;
        }
        start_index += 1;
    }
    while start_index < end_index {
        c = iter.next_back().unwrap_or(char::from_u32(start).unwrap());
        if c.is_numeric() {
            end = c.to_digit(10).expect("Line 38");
            eflag = true;
            break;
        }
        end_index -= 1;
    }

    if !eflag {
        end = start;
    }

    return (start, end);
}

fn append_nums(n: (u32, u32)) -> u32 {
    return n.0 * 10 + n.1;
}
