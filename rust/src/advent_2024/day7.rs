use std::fs::File;
use std::io::{BufRead, BufReader};

fn concat(lhs: i64, rhs: i64) -> Option<i64> {
    let str = format!("{}{}", lhs, rhs);
    str.parse::<i64>().ok()

}

fn is_valid(expect_number: i64, current_number: i64, numbers: &[i64], use_concat: bool) -> bool {
    if current_number == expect_number && numbers.is_empty() {
        return true;
    } else if expect_number < current_number {
        return false;
    }
    if !numbers.is_empty() {
        if let Some(new_number) = current_number.checked_add(numbers[0]) {
            if is_valid(expect_number, new_number, &numbers[1..], use_concat) {
                return true;
            }
        }
        if let Some(new_number) = current_number.checked_mul(numbers[0]) {
            if is_valid(expect_number, new_number, &numbers[1..], use_concat) {
                return true;
            }
        }
        if use_concat {
            if let Some(new_number) = concat(current_number, numbers[0]) {
                if (is_valid(expect_number, new_number, &numbers[1..], use_concat)) {
                    return true;
                }
            }
        }
    }
    false
}



pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split(':').collect();
        let expect_number: i64 = numbers[0].parse().ok().unwrap();
        let numbers: Vec<i64> = numbers[1].split_whitespace().map(|s| s.parse().ok().unwrap()).collect();
        if is_valid(expect_number, numbers[0], &numbers[1..], false) {
            result += expect_number;
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split(':').collect();
        let expect_number: i64 = numbers[0].parse().ok().unwrap();
        let numbers: Vec<i64> = numbers[1].split_whitespace().map(|s| s.parse().ok().unwrap()).collect();
        if is_valid(expect_number, numbers[0], &numbers[1..], true) {
            result += expect_number;
        }
    }
    result
}