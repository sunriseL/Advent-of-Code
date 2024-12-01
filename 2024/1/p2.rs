use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input")?; // Open the file
    let reader = BufReader::new(file);
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_map = HashMap::new();
    for line in reader.lines()
    {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();

        left_vec.push(words[0].parse().expect("parse string to i32 error"));
        let right_num: i32 = words[1].parse().expect("parse string to i32 error");
        let count = right_map.entry(right_num).or_insert(0);
        *count += 1;
    }
    let mut result : i32 = 0;
    for num in left_vec
    {
        result += num * right_map.get(&num).unwrap_or(&0);
    }
    println!("Result: {}", result);
    Ok(())
}
