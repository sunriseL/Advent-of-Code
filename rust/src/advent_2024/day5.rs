use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut parse_first = true;
    let mut result: i64 = 0;
    let mut m = HashSet::<(i32, i32)>::new();
    let mut updates : Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            parse_first = false;
            continue;
        }
        if parse_first {
            let numbers: Vec<i32> = line.split('|').filter_map(|s| s.parse().ok()).collect();
            m.insert((numbers[0], numbers[1]));
        } else {
            let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            updates.push(numbers);
        }
    }
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() {
            for j in i..update.len() {
                if m.contains(&(update[j], update[i])) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            result += i64::from(update[update.len() / 2]);
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut m = HashSet::<(i32, i32)>::new();
    let mut updates : Vec<Vec<i32>> = Vec::new();
    let mut parse_first = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            parse_first = false;
            continue;
        }
        if parse_first {
            let numbers: Vec<i32> = line.split('|').filter_map(|s| s.parse().ok()).collect();
            m.insert((numbers[0], numbers[1]));
        } else {
            let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            updates.push(numbers);
        }
    }
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() {
            for j in i..update.len() {
                if m.contains(&(update[j], update[i])) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            continue;
        }
        for i in 0..update.len() {
            let mut count: i32 = 0;
            for j in 0..update.len() {
                if i == j { continue; }
                if m.contains(&(update[i], update[j])) {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            if count == 0 {
                result += i64::from(update[i]);
                break;
            }
        }
    }
    result
}