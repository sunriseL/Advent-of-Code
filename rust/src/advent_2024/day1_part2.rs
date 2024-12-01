use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut left_vec: Vec<i64> = Vec::new();
    let mut right_map: HashMap<i64, i64> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();

        left_vec.push(words[0].parse().expect("parse string to i64 error"));
        let right_num: i64 = words[1].parse().expect("parse string to i64 error");
        let count = right_map.entry(right_num).or_insert(0);
        *count += 1;
    }
    left_vec
        .iter()
        .fold(0, |sum, num| sum + num * right_map.get(&num).unwrap_or(&0))
}
