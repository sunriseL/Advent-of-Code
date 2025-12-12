use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut input: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if input[i-1][j] == 'S' || input[i-1][j] == '|' {
                if input[i][j] == '^' {
                    result += 1;
                    input[i][j-1] = '|';
                    input[i][j+1] = '|';
                } else {
                    input[i][j] = '|';
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
    let mut input: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();
    let mut counts: Vec<Vec<i64>> = Vec::new();
    counts = vec![vec![0; input[0].len()]; input.len()];
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if input[i-1][j] == 'S' {
                counts[i][j] = 1;
                input[i][j] = '|';
            } else if input[i-1][j] == '|' {
                counts[i][j] += counts[i-1][j];
                if input[i][j] == '^' {
                    input[i][j-1] = '|';
                    input[i][j+1] = '|';
                    counts[i][j-1] += counts[i-1][j];
                    counts[i][j+1] += counts[i-1][j];
                } else {
                    input[i][j] = '|';
                }
            }
        }
    }
    result += counts.last().unwrap().iter().sum::<i64>();
    
    result
}
