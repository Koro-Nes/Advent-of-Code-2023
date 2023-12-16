use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, cmp::Ordering};


/*
strength definitions:
high card = 0
one pair = 1
two pair = 2
three of a kind = 3
full house = 4
four of a kind = 5
five of a kind = 6
*/
#[derive(Debug, Clone, PartialEq)]
struct Cards {
    cards: String,
    strength: u8,
    bid: u128
}

impl Cards {
    fn new(cards: String) -> Cards {
        let seperated = cards.split_ascii_whitespace().collect::<Vec<&str>>();

        let hand_type = get_hand_type(seperated[0].to_string());

        return Cards { cards: seperated[0].to_string(), strength: hand_type as u8, bid: seperated[1].parse::<u128>().unwrap() };
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.strength == other.strength {
            let hm = create_strength_map();
            let mut c1 = self.cards.chars().into_iter();
            let mut c2 = other.cards.chars().into_iter();

            loop {
                let a = match c1.next() {
                    Some(x) => x,
                    None => break,
                };
                let b = match c2.next() {
                    Some(x) => x,
                    None => break,
                };
                if a != b {
                    let a_strenght = hm.get(&a).unwrap();
                    let b_strength = hm.get(&b).unwrap();
                    if a_strenght < b_strength {
                        return Some(Ordering::Less);
                    } else  {
                        return Some(Ordering::Greater);
                    }
                }
            }
        } else {
            if self.strength < other.strength {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        }
        return Some(Ordering::Equal);
    }
}

fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);
    let input = get_input(buf);
    let mut cards = parse_input(input);
    sort_hands(&mut cards);
    let result = get_product(cards.clone());
    println!("Result: {}", result);
}

#[inline]
fn get_input(buf: BufReader<File>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for line in buf.lines() {
        res.push(line.unwrap());
    }
    return res;
}

#[inline]
fn parse_input(v: Vec<String>) -> Vec<Cards> {
    let mut res: Vec<Cards> = Vec::new();
    for str in v {
        let cards = Cards::new(str.to_string());
        res.push(cards);
    }
    return res;
}

#[inline]
fn get_hand_type(str: String) -> u8 {

    let mut hashmap: HashMap<char, u8> = HashMap::new();
    for c in str.chars() {
        if hashmap.contains_key(&c) {
            *hashmap.get_mut(&c).unwrap() += 1;
        } else {
            hashmap.insert(c, 1);
        }
    }

    let mut pairs = 0;
    let mut triple = 0;
    let mut jokers = 0;
    let mut curr_hand_strength: u8 = 0;

    for v in hashmap {
        if v.0 == 'J' {
            jokers = v.1;
        }
        match v.1 {
            1 => (),
            2 => pairs += 1,
            3 => triple += 1,
            4 => curr_hand_strength = 5, // four of a kind
            5 => return 6, // five of a kind
            _ => panic!("Invalid hand"),
        }
    }

    match pairs {
        0 => (),
        1 => {
            if triple == 1 {
                curr_hand_strength = 4 // full house
            } else {
                curr_hand_strength = 1 // one pair
            }
        },
        2 => curr_hand_strength = 2, // two pair
        _ => panic!("Not possible to have 3 pairs."),
    }
    match triple {
        0 => (),
        1 => if pairs < 1 { curr_hand_strength = 3 }, // three of a kind
        _ => panic!("Not possible to have 2 triples."),
    }
    if jokers > 0 {
        curr_hand_strength = get_improved_joker_hand(curr_hand_strength, jokers);
    }
    return curr_hand_strength;
}

#[inline]
fn sort_hands(v: &mut Vec<Cards>) {
    v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
}

#[inline]
fn create_strength_map() -> HashMap<char, u8> {
    let mut hm: HashMap<char, u8> = HashMap::new();
    hm.insert('J', 0);
    hm.insert('2', 1);
    hm.insert('3', 2);
    hm.insert('4', 3);
    hm.insert('5', 4);
    hm.insert('6', 5);
    hm.insert('7', 6);
    hm.insert('8', 7);
    hm.insert('9', 8);
    hm.insert('T', 9);
    hm.insert('Q', 10);
    hm.insert('K', 11);
    hm.insert('A', 12);
    return hm;
}

#[inline]
fn get_product(v: Vec<Cards>) -> u128 {
    let mut i: u128 = 1;
    let mut sum: u128 = 0;
    for c in v {
        sum += c.bid * i;
        i += 1;
    }
    return sum;
}

#[inline]
fn get_improved_joker_hand(curr_hand_strength: u8, jokers: u8) -> u8 {
    let improved_strength: u8;
    match curr_hand_strength {
        0 => improved_strength = 1, // 0 implies high card: ABCDJ => AABCD | one pair
        1 => improved_strength = 3, // 1 implies one pair:  AABCJ => AAABC | three of a kind
        2 => {
                if jokers < 2 {
                    improved_strength = 4; // 2 implies two pair:  AABBJ => AAABB | full house
                } else {
                    improved_strength = 5; // AAJJB => AAAAB | four of a kind
                }
            }
        3 => {
               match jokers {
                1 => improved_strength = 5, // AAAJB | four of a kind
                2 => improved_strength = 6, // AAAJJ | five of a kind
                3 => improved_strength = 5, // JJJAB | four of a kind
                _ => panic!("Invalid input."),
               } 
            },
        4 => improved_strength = 6, // 4 implies full house: AAAJJ || JJJAA => AAAAA | five of a kind
        5 => improved_strength = 6, // 5 implies four of a kind: AAAAJ => AAAAA | five of a kind
        _ => panic!("{}: Invalid input.", curr_hand_strength), 
    }
    return improved_strength;
}