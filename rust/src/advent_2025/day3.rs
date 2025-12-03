use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let voltage_lines: Vec<Vec<i64>> = reader
    .lines()
    .filter_map(|line| {
        let line = line.ok()?;
        let v = line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i64))
            .collect::<Vec<i64>>();
        Some(v)
    }).collect();
    for voltage_line in voltage_lines {
        let mut max_pos = 0;
        let mut max_v = i64::MIN;
        for pos in 0..voltage_line.len()-1 {
            if voltage_line[pos] > max_v {
                max_v = voltage_line[pos];
                max_pos = pos;
            }
        }
        
        let mut second_pos = 0;
        let mut second_v = i64::MIN;
        for pos in (max_pos + 1)..voltage_line.len() {    
            if voltage_line[pos] > second_v {
                second_v = voltage_line[pos];
                second_pos = pos;
            }
        }
        // let compact_line: String = voltage_line.iter().map(|d| d.to_string()).collect();
        // println!("voltage_line: {}", compact_line);
        // println!("max_v: {}, max_pos: {}, second_v: {}, second_pos: {}", max_v, max_pos, second_v, second_pos);
        result += max_v * 10 + second_v;
    }
    result
}

fn get_max(voltage_line: &Vec<i64>, start_pos: usize, end_pos: usize) -> (i64, usize) {
    let mut max_v = i64::MIN;
    let mut max_pos = 0;
    for pos in (start_pos)..end_pos {
        if voltage_line[pos] > max_v {
            max_v = voltage_line[pos];
            max_pos = pos;
        }
    }
    (max_v, max_pos)
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let voltage_lines: Vec<Vec<i64>> = reader
    .lines()
    .filter_map(|line| {
        let line = line.ok()?;
        let v = line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i64))
            .collect::<Vec<i64>>();
        Some(v)
    }).collect();
    for voltage_line in voltage_lines {
        let mut start_pos = 0;
        let mut r: i64 = 0;
        for i in 0..12 {
            let (max_v, max_pos) = get_max(&voltage_line, start_pos, voltage_line.len()-(11 - i));
            // println!("max_v: {}, max_pos: {}", max_v, max_pos);
            r = r * 10 + max_v;
            start_pos = max_pos + 1;
        }

        // println!("r: {}", r);
        result += r;
    }
    result
}
