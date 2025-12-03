use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let instructions: Vec<(char, i32)> = reader
    .lines()
    .filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        }
        let direction = line.chars().next()?;
        let distance = line[1..].parse::<i32>().ok()?;
        Some((direction, distance))
    })
    .collect();
    let count = instructions.iter().fold((50, 0), |(mut s, mut c), (direction, distance)| {
        if *direction == 'R' {
            s += distance;
        } else if *direction == 'L' {
            s -= distance;
        }
        s = ((s % 100) + 100) % 100;
        if s == 0 {
            c += 1;
        }
        (s, c)
    }).1;
    count
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let instructions: Vec<(char, i32)> = reader
    .lines()
    .filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        }
        let direction = line.chars().next()?;
        let distance = line[1..].parse::<i32>().ok()?;
        Some((direction, distance))
    })
    .collect();
    let count = instructions.iter().fold((50, 0), |(mut s, mut c), (direction, distance)| {
        c += distance / 100;
        let d = distance % 100;
        let prev_s = s;
        if *direction == 'R' {
            s += d;
        } else if *direction == 'L' {
            s -= d;
        }
        if s >= 100 || (prev_s > 0 && s <= 0) {
            c += 1;
        }
        s = ((s % 100) + 100) % 100;
        (s, c)
    }).1;
    count as i64
}
