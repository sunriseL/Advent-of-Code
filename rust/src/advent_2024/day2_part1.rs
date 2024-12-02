use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        let greater_list: Vec<bool> = numbers.iter().zip(numbers.iter().skip(1)).map(|(a, b)| b > a).collect();
        let distance_list: Vec<i32> = numbers.iter().zip(numbers.iter().skip(1)).map(|(a, b)| (b - a).abs()).collect();

        result += i64::from(greater_list.iter().all(|&x| x == greater_list[0]) && distance_list.iter().all(|&x| x >= 1 && x <= 3));
    }
    result
}
