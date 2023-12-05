use std::{fs::File, io::{BufRead, BufReader}, ops::Range};

#[derive(Debug)]
struct Entry {
    type_t: Type,
    value: Option<u32>,
    col: usize,
    row: Range<usize>
}

#[derive(Debug)]
struct Num {
    values: Vec<u32>,
}

impl Num {  
    fn get_value(self) -> u32 {
        let mut res: u32 = 0;
        let mut exp: u32 = (self.values.len() - 1).try_into().unwrap();
        for n in self.values {
            res += n * (10^exp);
            exp -= 1;
        }
        return res;
    }
}

impl Entry {
    fn new_num(value: u32, col: usize, row: Range<usize>) -> Entry {
        return Entry { type_t: Type::Number, value: Some(value), col, row };
    }
    fn new_sym(col: usize, row: usize) -> Entry {
        return Entry { type_t: Type::Symbol, value: None, col: col, row: row..row }
    }
    fn is_num(&self) -> bool {
        return matches!(self.type_t, Type::Number); 
    }
}

#[derive(Debug)]
enum Type {
    Number,
    Symbol
}


fn main() {

    let file: File = File::open("./test.txt").unwrap(); //TODO: Change to input.txt
    let buf = BufReader::new(file);

    let mut grid_vec: Vec<Vec<Entry>> = read_input(buf); 
    for g in grid_vec {
        for e in g {
            println!("{:?}", e);
        }
    }
    

}

fn read_input(buf: BufReader<File>) -> Vec<Vec<Entry>> {
    let mut res: Vec<Vec<Entry>> = Vec::new();
    let mut col: usize = 0;

    for l in buf.lines() {

        let string = l.unwrap();
        let curr_entries = convert_line(string, col);
        let reduced_entries = reduce_vec(curr_entries, col);
        res.push(reduced_entries);
        col += 1;
    }
    return res;
}

fn convert_line(s: String, col: usize) -> Vec<Entry> {

    let mut entry_vec: Vec<Entry> = Vec::new();
    let mut row: usize = 0;

    for c in s.chars() {

        if c != '.' {
            let e: Entry;
            if c.is_digit(10) {
                e = Entry::new_num(c.to_digit(10).unwrap(), col, row..row);
            } else {
                e = Entry::new_sym(col, row);
            }
            entry_vec.push(e);
        }

        row += 1;
    }
    let res_vec = reduce_vec(entry_vec, col);


    return res_vec;
}


// TODO: Add comparsion by range, not by location in vector.

fn reduce_vec(v: Vec<Entry>, col: usize) -> Vec<Entry> {

    let mut res_vec: Vec<Entry> = Vec::new();
    let mut curr_num: Vec<u32> = Vec::new();
    let mut row: usize = 0;

    let mut start: usize;
    let mut end: usize = 0;

    for e in v {
        if e.is_num() {
            println!("Found Symbol: {:?}", e);
            curr_num.push(e.value.unwrap());
            end = row;
        } else {
            // add last combined number
            if !curr_num.is_empty() {
                println!("Number parsing end.");
                start = row + 1;
                let curr_num_value = vec_to_value(&curr_num);
                let curr_num_entry = Entry::new_num(curr_num_value, col, start..end);
                res_vec.push(curr_num_entry);
                curr_num.clear();
            }
            res_vec.push(e);
        }
        row += 1;
    }
    return res_vec;
}

fn vec_to_value(v: &Vec<u32>) -> u32 {
    let mut res: u32 = 0;
    let mut exp: u32 = (v.len()).try_into().unwrap();
    println!("{:?}", v);
    for n in v {
        res += n * (10^exp);
        exp -= 1;
    }
    return res;
}