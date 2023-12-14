use std::{fs::File, io::{BufRead, BufReader}, ops::Range, fmt::Display};

#[derive(Debug, Clone)]
struct Entry {
    type_t: Type,
    value: Option<u32>,
    col: usize,
    row: Range<usize>
}

impl Entry {
    fn new_num(value: u32, col: usize, row: Range<usize>) -> Entry {
        return Entry { type_t: Type::Number, value: Some(value), col, row };
    }
    fn new_sym(col: usize, row: usize) -> Entry {
        return Entry { type_t: Type::Symbol, value: None, col: col, row: row..row }
    }
    fn new_dot(col: usize, row: usize) -> Entry {
        return Entry { type_t: Type::Dot, value: None, col, row: row..row }
    }
    fn is_num(&self) -> bool {
        return matches!(self.type_t, Type::Number); 
    }
    fn is_symbol(&self) -> bool {
        return matches!(self.type_t, Type::Symbol);
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {:?}, Value: {:?}, Row: {}..{}, Col: {}", self.type_t, self.value, self.row.start, self.row.end, self.col)
    }
}

#[derive(Debug, Clone)]
enum Type {
    Number,
    Symbol,
    Dot,
}


fn main() {

    let file: File = File::open("./input.txt").unwrap(); 
    let buf = BufReader::new(file);

    let grid_vec: Vec<Vec<Entry>> = read_input(buf); 

    let part_numbers: Vec<Entry> = get_part_nums(grid_vec);
    
    let sum: u32 = sum_numbers(part_numbers);
    println!("Sum is: {}", sum);
}

fn read_input(buf: BufReader<File>) -> Vec<Vec<Entry>> {
    let mut res: Vec<Vec<Entry>> = Vec::new();
    let mut col: usize = 0;

    for l in buf.lines() {

        let string = l.unwrap();
        let curr_entries = convert_line(string, col);
        if !curr_entries.is_empty() {
            res.push(curr_entries);
        }
        col += 1;
    }

    return res;
}

fn convert_line(s: String, col: usize) -> Vec<Entry> {

    let mut entry_vec: Vec<Entry> = Vec::new();
    let mut row: usize = 0;

    for c in s.chars() {
        let e: Entry;
        if c.is_digit(10) {
            e = Entry::new_num(c.to_digit(10).unwrap(), col, row..row);
        } else if c == '.' {
            e = Entry::new_dot(col, row);
        } else {
            e = Entry::new_sym(col, row);
        } 
        row += 1;
        entry_vec.push(e);
    }
    
    let res_vec = reduce_vec(entry_vec, col);

    return res_vec;
}

fn reduce_vec(v: Vec<Entry>, col: usize) -> Vec<Entry> {

    let mut res_vec: Vec<Entry> = Vec::new();
    let mut curr_num: Vec<u32> = Vec::new();

    let mut num_start: usize = 0;
    let mut num_end: usize = 0;
    let mut row: usize = 0;

    let mut number_being_assembled = false;


    for entry in v {

        if entry.is_num() {
            if number_being_assembled {
                num_end = row;
                if !curr_num.is_empty() {
                    curr_num.push(entry.value.unwrap());
                }  else {
                    panic!("Huh");
                }
            } else {
                number_being_assembled = true;
                num_start = row;
                num_end = row;
                curr_num.push(entry.value.unwrap());
            }
        } else {
            if number_being_assembled {
                number_being_assembled = false;
                let val = vec_to_value(&curr_num);
                curr_num.clear();
                if num_end == 0 { num_end = num_start }
                let range: Range<usize> = num_start..num_end;
                let res_num = Entry { type_t: Type::Number, value: Some(val), col: col, row: range };
                res_vec.push(res_num);
                if entry.is_symbol() {
                    res_vec.push(entry);
                }
            } else {
                if entry.is_symbol() {
                    res_vec.push(entry);
                }
            }
        }
        row += 1;
    }
    if !curr_num.is_empty() {
        res_vec.push(Entry { type_t: Type::Number, value: Some(vec_to_value(&curr_num)), col: col, row: num_start..num_end });
    } 

    return res_vec;
}

fn vec_to_value(v: &Vec<u32>) -> u32 {
    let mut res: u32 = 0;
    let len: u32 = v.len().try_into().unwrap();
    let mut exp: u32 = len;
    for n in v {
        exp -= 1;
        res += n * 10_u32.pow(exp);
    }
    return res;
}

fn get_part_nums(input: Vec<Vec<Entry>>) -> Vec<Entry> {
    
    let mut res_vec: Vec<Entry> = Vec::new();

    for line in &input {
        for entry in line {
            if entry.is_num() {
                if has_symbol(&input, entry.col, &entry.row) {
                    res_vec.push(entry.clone());
                }
            }
        }
    }
    return res_vec;
}

fn has_symbol(v: &Vec<Vec<Entry>>, col: usize, range: &Range<usize>) -> bool {

    let mut col_counter: usize = 0;


    for line in v {
        if (col > 0 && col_counter == col - 1) || col_counter == col || col_counter == col + 1 {
            for entry in line {
                if entry.is_symbol() {
                    if check_ranges(range, &entry.row) {
                        return true;
                    }
                }
            }
        }
        col_counter += 1;
    }
    return false;
}

// r1 needs to be number, r2 needs to be symbol
fn check_ranges(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    return i32::try_from(r2.start).unwrap() >= i32::try_from(r1.start).unwrap() - 1 && r2.start <= r1.end + 1; 
}

fn sum_numbers(v: Vec<Entry>) -> u32{
    let mut sum:  u32 = 0;
    for e in v {
        if e.is_symbol() {
            panic!("Entry not a number.");
        }
        sum += e.value.unwrap();
    }
    return sum;
}