use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn split(number: i64) -> Option<(i64, i64)> {
    let mut scale: i64 = 10;
    while number >= scale {
        let left = number / scale;
        let right = number % scale;
        if left >= (scale / 10) && left <= (scale - 1) {
            // println!("Split {} to {} and {}", number, left, right);
            return Some((left, right))
        }
        scale *= 10;
    }
    None
}

fn rock_count(m: &mut HashMap<(i64, i64), i64>, number: i64, left_count: i64) -> i64 {
    if m.contains_key(&(number, left_count)) {
        let result = *m.get(&(number, left_count)).unwrap();
        // println!("Get ({},{}) from cache: {}", number, left_count, result);
        return result;
    }
    if left_count == 0 {
        return 1;
    }
    let result: i64;
    if number == 0 {
        result = rock_count(m, 1, left_count - 1);
    } else if let Some((left, right)) = split(number) {
        result = rock_count(m, left, left_count - 1) + rock_count(m, right, left_count - 1);
    } else {
        result = rock_count(m, number * 2024, left_count - 1);
    }
    m.insert((number, left_count), result);
    // println!("Set ({},{}) to cache: {}", number, left_count, result);
    result
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut m = HashMap::<(i64, i64), i64>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        for number in numbers {
            result += rock_count(&mut m, number, 25);
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut m = HashMap::<(i64, i64), i64>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        for number in numbers {
            result += rock_count(&mut m, number, 75);
        }
    }
    result
}