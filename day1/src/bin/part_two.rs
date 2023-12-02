use std::{fs::File, io::{BufReader, BufRead}};


fn main() {

    let file = File::open("./input.txt").expect("Invalid input path");
    let buf_reader = BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();
    let mut res_vec: Vec<u32> = Vec::new();

    run_tests();

    for line in buf_reader.lines() {
        let string = line.unwrap().trim().to_string();
        numbers = parse_numbers(string);
        if numbers.len() != 0 {
            res_vec.push(append_numbers((numbers[0], *numbers.last().unwrap())));
        }
    }

    let result: u32 = res_vec.iter().sum();
    println!("Result is: {}", result);

}

fn parse_numbers(str: String) -> Vec<u32> {

    let char_vec: Vec<char> = str.chars().into_iter().collect();
    let mut res_vec: Vec<u32> = Vec::new();
    let max_index = char_vec.len();

    for mut i in 0..max_index {
        let curr = char_vec[i];
        if curr.is_digit(10) {
            res_vec.push(curr.to_digit(10).expect("Line 14."));
        } else {
                let opt_num = get_num_from_string(&char_vec, i, max_index);
                if opt_num.0.is_some() {
                    res_vec.push(opt_num.0.unwrap());
                    i += opt_num.1.unwrap();
                }
        } 
    }

    return res_vec;
}

fn get_num_from_string(vec: &Vec<char>, i: usize, len: usize) -> (Option<u32>, Option<usize>) {

    let ip1: usize = i+1;
    let ip2: usize = i+2;
    let ip3: usize = i+3;
    let ip4: usize = i+4;

    let can_3 = (len as i32) - (i as i32) >= 2;
    let can_4 = (len as i32) - (i as i32) >= 3;
    let can_5 = (len as i32) - (i as i32) >= 4;

    if !can_3 {
        return (None, None);
    }



    if can_3 && vec[i] == 'o' && vec[ip1] == 'n' && vec[ip2] == 'e' { //1
        return (Some(1), Some(3));
    }
    if vec[i] == 't' {
        if can_3 && vec[ip1] == 'w' && vec[ip2] == 'o' { //2
            return (Some(2), Some(3));
        }
        if can_5 && vec[ip1] == 'h' && vec[ip2] == 'r' && vec[ip3] == 'e' && vec[ip4] == 'e' { //3
            return (Some(3), Some(4));
        }
    }
    if can_4 && vec[i] == 'f' {
        if vec[ip1] == 'o' && vec[ip2] == 'u' && vec[ip3] == 'r' { //4
            return (Some(4), Some(4));
        }
        if vec[ip1] == 'i' && vec[ip2] == 'v' && vec[ip3] == 'e' { //5
            return (Some(5), Some(4));
        }
    }
    if vec[i] == 's' {
        if can_3 && vec[ip1] == 'i' && vec[ip2] == 'x' { //6
            return (Some(6), Some(3));
        }
        if can_5 && vec[ip1] == 'e' && vec[ip2] == 'v' && vec[ip3] == 'e' && vec[ip4] == 'n' { //7
            return (Some(7), Some(5));
        }
    }
    if can_5 && vec[i] == 'e' && vec[ip1] == 'i' && vec[ip2] == 'g' && vec[ip3] == 'h' && vec[ip4] == 't' { //8
        return (Some(8), Some(5));
    }
    if can_4 && vec[i] == 'n' && vec[ip1] == 'i' && vec[ip2] == 'n' && vec[ip3] == 'e' { //9
        return (Some(9), Some(4));
    }    

    return (None, None);
}

fn append_numbers(c: (u32, u32)) -> u32 {

    return c.0 * 10 + c.1;
}

// Tests
#[allow(dead_code)]
fn run_tests() {

    let s1 = String::from("sixtqqszg6fourzbjhkvlkmnkdztq");
    println!("Input String: {}", s1);
    let n1 = parse_numbers(s1);
    println!("Result Vector: {:?}", n1);
    let r1 = append_numbers((n1[0], *n1.last().unwrap()));
    println!("Expected: 64, Result: {}", r1);
    let b1 = 64 == r1;
    
    print!("\n");

    let s2 = String::from("flcqsevenlgvtnvnctpfjvrlg65dkdbjn9");
    println!("Input String: {}", s2);
    let n2 = parse_numbers(s2);
    println!("Result Vector: {:?}", n2);
    let r2 = append_numbers((n2[0], *n2.last().unwrap()));
    println!("Expected: 79, Result: {}", r2);
    let b2 = 79 == r2;


    print!("\n");


    let s3 = String::from("four498");
    println!("Input String: {}", s3);
    let n3 = parse_numbers(s3);
    println!("Result Vector: {:?}", n3);
    let r3 = append_numbers((n3[0], *n3.last().unwrap()));
    println!("Expected: 48, Result: {}", r3);
    let b3 = 48 == r3;


    print!("\n");


    let s4 = String::from("eight5one43nmkxdseight5");
    println!("Input String: {}", s4);
    let n4 = parse_numbers(s4);
    println!("Result Vector: {:?}", n4);
    let r4 = append_numbers((n4[0], *n4.last().unwrap()));
    println!("Expected: 85, Result: {}", r4);
    let b4 = 85 == r4;


    print!("\n");


    let s5 = String::from("6pjfz9twofive1bfdseven");
    println!("Input String: {}", s5);
    let n5 = parse_numbers(s5);
    println!("Result Vector: {:?}", n5);
    let r5 = append_numbers((n5[0], *n5.last().unwrap()));
    println!("Expected: 67, Result: {}", r5);
    let b5 = 67 == r5;


    print!("\n");


    let s6 = String::from("two6sevennine27three78");
    println!("Input String: {}", s6);
    let n6 = parse_numbers(s6);
    println!("Result Vector: {:?}", n6);
    let r6 = append_numbers((n6[0], *n6.last().unwrap()));
    println!("Expected: 28, Result: {}", r6);
    let b6 = 28 == r6;


    print!("\n");

    let s7 = String::from("37");
    println!("Input String: {}", s7);
    let n7 = parse_numbers(s7);
    println!("Result Vector: {:?}", n7);
    let r7 = append_numbers((n7[0], *n7.last().unwrap()));
    println!("Expected: 37, Result: {}", r7);
    let b7 = 37 == r7;

    print!("\n");

    let s8 = String::from("1");
    println!("Input String: {}", s8);
    let n8 = parse_numbers(s8);
    println!("Result Vector: {:?}", n8);
    let r8 = append_numbers((n8[0], *n8.last().unwrap()));
    println!("Expected: 11, Result: {}", r8);
    let b8 = 11 == r8;



    let b = count_tests(vec![b1, b2, b3, b4, b5, b6, b7, b8]);
    let t = b.0;
    let a = b.1;
    println!("Result: {b1}, {b2}, {b3}, {b4}, {b5}, {b6}, {b7}, {b8}   |  {a}/{t}\n");
}

fn count_tests(b: Vec<bool>) -> (u8, u8) {
    let length = b.len() as u8;
    let mut count: u8 = 0;

    for c in b {
        if c == true {
            count += 1;
        }
    }
    return (length, count);
}