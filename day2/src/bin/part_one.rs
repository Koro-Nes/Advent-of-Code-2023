use std::{io::{BufReader, BufRead}, fs::File};

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;


#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>
}

impl Game {
    fn is_satisfactory(&self) -> bool {
        for s in &self.sets {
            if !s.check() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn check(&self) -> bool {
        return self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE;
    }
}

fn main() {

    let file = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut games: Vec<Game> = Vec::new();

    for line in buf.lines() {
        let separated_sets = separate_sets_by_string(&line.unwrap());
        let uid_sets = poll_id_from_string(&separated_sets);
        let id = uid_sets.0;
        let game = parse_game(uid_sets.1, id);
        games.push(game);
    }
    let mut games_satisfied: Vec<Game> = Vec::new();
    for g in games {
        if g.is_satisfactory() {
            games_satisfied.push(g);
        }
    }
    let sum = sum_games(games_satisfied);
    println!("Sum: {}", sum);

}

fn separate_sets_by_string(string: &String) -> Vec<String> {
    let sep_split = string.split_terminator(";");
    let mut res_vec: Vec<String> = Vec::new();

    for set in sep_split {
        let set_str = set.replace(":", "").replace(",", "");
        res_vec.push(set_str);
    }

    return res_vec;
}

fn poll_id_from_string(vec: &Vec<String>) -> (u32, Vec<String>) {
    let game_string = &vec[0];
    let mut result_vec: Vec<String> = Vec::new();
    let id: u32;

    let ws_split = game_string.split_ascii_whitespace();
    let mut only_set_string = String::from("");

    //extract id value
    let game_vec: Vec<&str> = ws_split.into_iter().collect();
    id = game_vec[1].parse().unwrap();
    for i in 2..game_vec.len() {
        let mut entry: String = game_vec[i].to_string();
        entry.push_str(" ");
        only_set_string.push_str(entry.as_str());
    }

    //create new vector that doesn't contain the game id
    result_vec.push(only_set_string);
    for v in vec {
        if !v.contains("Game") {
            result_vec.push(v.to_string());
        } 
    }
    return (id, result_vec);
}

fn parse_set(string: &String) -> Set {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let string_vec: Vec<&str> = string.split_ascii_whitespace().collect();
    
    let mut str_iter = string_vec.iter();

    for _i in 0..str_iter.len()/2 {
        let curr_val = str_iter.next().unwrap();
        let curr_col = str_iter.next().unwrap();

        match *curr_col {
            "red" => red = curr_val.parse().unwrap(),
            "green" => green = curr_val.parse().unwrap(),
            "blue" => blue = curr_val.parse().unwrap(),
            _ => println!("Invalid arg: {}", curr_col)
        }
    }

    return Set { red, green, blue }
}

fn parse_game(strings: Vec<String>, id: u32) -> Game {
    let mut sets: Vec<Set> = Vec::new();

    for s in strings {
        let set = parse_set(&s);
        sets.push(set);
    }

    return Game { id, sets }
}


fn sum_games(games: Vec<Game>) -> u32 {
    let mut sum = 0;
    for g in games {
        sum += g.id;
    }
    return sum;
}