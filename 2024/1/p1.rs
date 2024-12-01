use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?; // Open the file
    let reader = BufReader::new(file);
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    for line in reader.lines()
    {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();

        left_vec.push(words[0].parse().expect("parse string to i32 error"));
        right_vec.push(words[1].parse().expect("parse string to i32 error"));
    }
    left_vec.sort();
    right_vec.sort();
    let mut result : i32 = 0;
    for (l, r) in left_vec.iter().zip(right_vec.iter())
    {
        result += (l-r).abs();
    }
    println!("Result: {}", result);
    Ok(())
}
