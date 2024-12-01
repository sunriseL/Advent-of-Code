use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut left_vec: Vec<i64> = Vec::new();
    let mut right_vec: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();

        left_vec.push(words[0].parse().expect("parse string to i64 error"));
        right_vec.push(words[1].parse().expect("parse string to i64 error"));
    }
    left_vec.sort();
    right_vec.sort();
    left_vec
        .iter()
        .zip(right_vec.iter())
        .fold(0, |sum, (l, r)| sum + (l - r).abs())
}
