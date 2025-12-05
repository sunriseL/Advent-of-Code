use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_element(matrix: &Vec<Vec<char>>, r: i32, c: i32, e: char) -> bool {
    if r < 0 || r >= matrix.len() as i32 || c < 0 || c >= matrix[0].len() as i32 {
        return false;
    }
    if matrix[r as usize][c as usize] == e {
        return true;
    }
    false
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut matrix = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if is_element(&matrix, r as i32, c as i32, '@') {
                let mut count = 0;
                for dr in [-1, 0, 1].iter() {
                    for dc in [-1, 0, 1].iter() {
                        if *dr == 0 && *dc == 0 {
                            continue;
                        }
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if is_element(&matrix, nr, nc, '@') {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }   
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result:i64 = 0;
    let mut matrix = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }
    while true {
        let mut changed = false;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if is_element(&matrix, r as i32, c as i32, '@') {
                    let mut count = 0;
                    for dr in [-1, 0, 1].iter() {
                        for dc in [-1, 0, 1].iter() {
                            if *dr == 0 && *dc == 0 {
                                continue;
                            }
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if is_element(&matrix, nr, nc, '@') {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        result += 1;
                        changed = true;
                        matrix[r][c] = '.';
                    }
                }
            }
        }   
        if !changed {
            break;
        }
    }
    result
}
