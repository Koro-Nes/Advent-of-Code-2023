use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, sync::{Mutex, Arc}};
use rayon::prelude::*;

fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);
    let input = read_input(buf);

    let instructions = String::from(&input[0]);
    let mut hashmap: HashMap<String, (String, String)> = HashMap::new();
    insert_input_into_hashmap(input[2..input.len()].to_vec(), &mut hashmap);
    let start_nodes: Vec<&String> = hashmap.keys().filter(|x| x.ends_with("A")).collect();
    
    let steps_for_cycle = Arc::new(Mutex::new(Vec::new()));

    start_nodes.par_iter().for_each(|x| steps_for_cycle.lock().unwrap().push(get_z_node(&hashmap, instructions.chars().collect(), x))); 

    println!("{}", lcm_of_list(steps_for_cycle.lock().unwrap().to_vec())); 
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

fn get_z_node(hm: &HashMap<String, (String, String)>, instructions: Vec<char>, start: &String) -> u128 {
    
    let mut steps: u128 = 0;
    let mut curr_node = start;
    let mut instruction_cycle = instructions.iter().cycle();

    loop {
        steps += 1;
        let next_dir = instruction_cycle.next().unwrap();
        match next_dir {
            'L' => curr_node = &hm.get(curr_node).unwrap().0,
            'R' => curr_node = &hm.get(curr_node).unwrap().1,
            _ => panic!("Invalid Direction."),
        }
        if curr_node.ends_with("Z") {
            return steps;
        }
    }
}

fn lcm_of_list(v: Vec<u128>) -> u128 {
    let mut v_iter = v.iter();
    let first = *v_iter.next().unwrap();
    let second = *v_iter.next().unwrap();

    let mut res = lcm(first, second);

    while let Some(&next) = v_iter.next() {
        res = lcm(res, next);
    }

    return res;
}

fn lcm(a: u128, b: u128) -> u128 {
    return a * b / gcd(a, b);
}


fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    } else {
        gcd(b, a % b)
    }
}