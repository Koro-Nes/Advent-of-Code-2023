use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    picks: Vec<u32>,
}


fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);    
    let cards: Vec<Card> = read_input(buf); 
    let points: Vec<u32> = check_cards(&cards);
    let sum: u32 = points.iter().sum();
    println!("Sum: {}", sum);
}

fn read_input(buf: BufReader<File>) -> Vec<Card> {
    let mut res_vec: Vec<Card> = Vec::new();

    for line in buf.lines() {
        let curr_card = parse_line(line.unwrap());
        res_vec.push(curr_card);
    }

    return res_vec;
}

fn parse_line(line: String) -> Card {
    let seperate_hands: Vec<&str> = line.split('|').collect();
    let winning_nums_string = seperate_hands[0].to_string();
    let picked_nums_string = seperate_hands[1].to_string();

    //parse winning nums
    let mut winning_nums: Vec<u32> = Vec::new();
    {
        let seperate_name_from_values: Vec<&str> = winning_nums_string.split(':').collect();
        let value_string = seperate_name_from_values[1];
        let single_value_strings: Vec<&str> = value_string.trim().split(" ").collect();
        for val in single_value_strings {
            if !val.trim().is_empty() {
                winning_nums.push(val.parse().unwrap());
            }
        }
    }
    // parse picked nums
    let mut picks: Vec<u32> = Vec::new();
    {
        let value_string: Vec<&str> = picked_nums_string.trim().split(" ").collect();
        for val in value_string {
            if !val.trim().is_empty() {
                picks.push(val.parse().unwrap());
            }
        }
    }
    return Card { winning: winning_nums, picks: picks };
}

fn check_cards(cards: &Vec<Card>) -> Vec<u32> {
    let mut instances = create_instance_array(cards);
    println!("[Instance Array Uninitialized]: {:?}", instances);
    let mut i: usize = 0;
    for c in cards {
        let number_of_copies = check_points(c);
        for j in (i+1)..(i+1)+(number_of_copies as usize) {
            instances[j] += 1 * instances[i];
        }
        i += 1;
    } 

    println!("[Instance Array Initialized]: {:?}", instances);
    return instances;
}

fn create_instance_array(vec: &Vec<Card>) -> Vec<u32> {
    let mut res_vec: Vec<u32> = Vec::new();
    for _card in vec {
        res_vec.push(1);
    }
    return res_vec;
}

fn check_points(card: &Card) -> u32 {
    let mut points: u32 = 0;
    let mut map: HashMap<u32, u32> = HashMap::new();
    for n in &card.winning {
        map.insert(*n, *n);
    }
    for pick in &card.picks  {
        if map.contains_key(pick) {
            points += 1;
        }
    }
    return points;
}