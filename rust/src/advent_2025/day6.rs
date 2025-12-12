use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let mut numbers : Vec<Vec<i64>> = Vec::new();
    for line in lines.iter().take(4) {
        numbers.push(line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect());
    }
    let mut operators = Vec::<char>::new();
    for line in lines.iter().skip(4) {
        operators = line.split(' ').filter_map(|s| s.parse::<char>().ok()).collect();
        break;
    }
    // println!("numbers: {:?}", numbers);
    // println!("len: {}", numbers[0].len());
    // println!("operators: {:?}", operators);
    // println!("len: {}", operators.len());
    let mut result: i64 = 0;

    for i in 0..numbers[0].len() {
        if operators[i] == '+' {
            result += numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i];
        } else if operators[i] == '*' {
            result += numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i];
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let lines_chars = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut result:i64 = 0;
    let mut operators = Vec::<char>::new();
    let mut numbers : Vec<Vec<i64>> = Vec::new();
    for line in lines.iter().take(4) {
        numbers.push(line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect());
    }
    for line in lines.iter().skip(4) {
        operators = line.split(' ').filter_map(|s| s.parse::<char>().ok()).collect();
        break;
    }
    let mut max_digits: Vec<i64> = Vec::new();
    for i in 0..numbers[0].len() {
        let max_num = numbers[0][i].max(numbers[1][i]).max(numbers[2][i]).max(numbers[3][i]);
        // INSERT_YOUR_CODE
        let num_digits = if max_num == 0 {
            1
        } else {
            ((max_num as f64).abs().log10().floor() as i64) + 1
        };
        max_digits.push(num_digits);
    }
    // println!("max_digits: {:?}", max_digits);
    // println!("line_chars: {:?}", lines_chars);
    let mut pos: usize = 0;
    for (i, max_digit) in max_digits.iter().enumerate() {
        let mut num: i64 = 0;
        let mut processed_nums: Vec<i64> = Vec::new();
        // println!("max_digit: {}", max_digit);
        for j in (0..*max_digit).rev() {
            // println!("j: {}", j);
            num = 0;
            for index in 0..=3 {
                // println!("lines_chars[{}][{}]: {}", index, pos + j as usize, lines_chars[index][pos + j as usize]);
                if lines_chars[index][pos + j as usize] != ' ' {
                    num = num * 10 + (lines_chars[index][pos + j as usize] as i64 - '0' as i64);
                }
            }
            processed_nums.push(num);
        }
        // println!("processed_nums: {:?}", processed_nums);

        if operators[i] == '+' {
            result += processed_nums.iter().sum::<i64>();
        } else if operators[i] == '*' {
            result += processed_nums.iter().product::<i64>();
        }

        pos += *max_digit as usize + 1;
    }
    result
}
