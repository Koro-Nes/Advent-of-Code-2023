use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file: File = File::open("./test.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);
    let mut string_vec = Vec::new();
    for line in buf.lines() {
        string_vec.push(line.unwrap());
    }
    let transposed = transpose(&string_vec);
    let res_col = check_cols(&transposed);
    let res_rows = check_rows(&string_vec);
}

fn check_rows(v: &Vec<String>) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = Vec::new();
    for i in 0..v.len()-1 {
        if v[i] == v[i+1] {
            res.push(((i + 1) as u32, (i + 2) as u32));
        }
    }
    res
}

fn check_cols(v: &Vec<String>) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = Vec::new();

    // get transposed String vector
    let transposed = transpose(v);

    for l in 0..transposed.len() {
        for n in l+1..transposed.len() {
            if transposed[l] == transposed[n] {
                res.push(((l+1) as u32, (n+1) as u32));
            }
        }
    }

    res
}

fn transpose(v: &Vec<String>) -> Vec<String> {
    let grid: Vec<Vec<char>> = v
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut transposed_grid: Vec<Vec<char>> = Vec::new();

    for j in 0..grid[0].len() {
        let mut curr_line: Vec<char> = Vec::new();
        for i in 0..grid.len() {
            curr_line.push(grid[i][j]);        
        }
        transposed_grid.push(curr_line.clone());
        curr_line.clear();
    }

    let mut res = Vec::new();

    for line in transposed_grid {
        let mut curr_string = String::new();
        for c in line {
            curr_string.push(c);
        }
        res.push(curr_string);
    }
    return res;
}


#[cfg(test)]
mod test {
    use crate::{check_rows, check_cols, transpose};


    const PATTERN_1: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

    const PATTERN_2: &str = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_row_detection() {
        
        // pattern 1
        println!("Testing pattern 1");
        let v = PATTERN_1.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let expected: Vec<(u32, u32)> = vec![(2,6), (3,4)];
        let result = check_rows(&v);
        println!("exp: {:?}", expected);
        println!("res: {:?}", result);
        for pair in result {
            assert!(expected.contains(&pair));
        }

        // pattern 2
        println!("Testing pattern 2");
        let v = PATTERN_2.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let expected: Vec<(u32, u32)> = vec![(4,5), (3,6), (2, 7)];
        let result = check_rows(&v);
        println!("exp: {:?}", expected);
        println!("res: {:?}", result);
        for pair in result {
            assert!(expected.contains(&pair));
        }
    }

    #[test]
    fn test_col_detection() {

        //testing pattern 1
        println!("Testing pattern 1");
        let v = PATTERN_1.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = check_cols(&v);
        let expected: Vec<(u32, u32)> = vec![(5,6), (2,9), (3,8), (4,7)];
        println!("res: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result.len() == expected.len(), "Length differs");
        for pair in result {
            assert!(expected.contains(&pair), "Invalid pair");
        }

        //testing pattern 2
        println!("Testing pattern 2");
        let v = PATTERN_2.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = check_cols(&v);
        let expected: Vec<(u32, u32)> = vec![(3,4), (3,7), (3,8), (4,7), (4,8), (7,8)];
        println!("res: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result.len() == expected.len(), "Length differs");
        for pair in result {
            assert!(expected.contains(&pair), "Invalid pair");
        }

}

    #[test]
    fn test_transpose() {
        let v = vec![String::from("#.##..##."), String::from("..#.##.#."), String::from("##......#"),
            String::from("##......#"), String::from("..#.##.#."), String::from("..##..##."), String::from("#.#.##.#.")];

        let expected = vec![String::from("#.##..#"), String::from("..##..."), String::from("##..###"),
            String::from("#....#."), String::from(".#..#.#"), String::from(".#..#.#"), String::from("#....#."),
            String::from("##..###"), String::from("..##...")];
        let result = transpose(&v);
        assert!(expected == result, "Transpose test failed");

    }


}