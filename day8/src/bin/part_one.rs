use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);
    let input = read_input(buf);

    let instructions = String::from(&input[0]);
    let mut hashmap: HashMap<String, (String, String)> = HashMap::new();
    insert_input_into_hashmap(input[2..input.len()].to_vec(), &mut hashmap);

    let steps = find_zzz("AAA".to_string(), instructions, &hashmap);
    println!("Steps: {}", steps);
}

#[inline]
fn read_input(buf: BufReader<File>) -> Vec<String> {
    let mut res_vec: Vec<String> = Vec::new();
    for line in buf.lines() {
        res_vec.push(line.unwrap());
    }
    return res_vec;
}


//inserts values into hashmap and returns first node name
#[inline]
fn insert_input_into_hashmap(v: Vec<String>, hm: &mut HashMap<String, (String, String)>) -> String {
    let mut first: String = String::new();
    for line in v {
        let sep_str = line.split_terminator("=").collect::<Vec<&str>>();
        let key = sep_str[0].trim().to_string();
        if first.is_empty() {
            first = String::from(&key);
        }
        let sep_val_str = sep_str[1].trim().replace("(", "").replace(")", "");
        let sep_val = sep_val_str.split_terminator(",").collect::<Vec<&str>>(); 
        let val_l = sep_val[0].trim().to_string();
        let val_r = sep_val[1].trim().to_string();
        hm.insert(key, (val_l, val_r));
    }
    return first;
}

#[inline]
fn find_zzz(start: String, instructions: String, hm: &HashMap<String, (String, String)>) -> u32 {
    
    let mut counter = 0;
    let mut index = 0;

    let mut next: String = String::from(start);
    let instruction_vec = instructions.chars().collect::<Vec<char>>();

    loop {
        
        match instruction_vec[index] {
            'L' => {
                let next_instr = &hm[&next].0;
                next = String::from(next_instr);
                if next == "ZZZ".to_string() {
                    return counter + 1;
                }
            }
            'R' => {
                let next_instr = &hm[&next].1;
                next = String::from(next_instr);
                if next == "ZZZ".to_string() {
                    return counter + 1;
                }
            }
            _ => panic!("[find_zzz]: Invalid direction."),
        }

        if index + 1 >= instructions.len() {
            index = 0;
        } else {
            index += 1;
        }
        counter += 1;
    }
}