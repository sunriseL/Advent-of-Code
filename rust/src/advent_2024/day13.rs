use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_numbers(input: &str) -> (i64, i64) {
    let numbers: Vec<i64> = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    (numbers[0], numbers[1])
}

fn min_count(
    steps: ((i64, i64), (i64, i64)),
    ex: i64,
    ey: i64,
    cache: &mut HashMap<(i64, i64), Option<i64>>,
) -> Option<i64> {
    if let Some(count) = cache.get(&(ex, ey)) {
        return *count;
    }
    if ex < 0 || ey < 0 {
        return None;
    }
    if ex == 0 && ey == 0 {
        return Some(0);
    }
    // println!("steps: {:?}", steps);
    // println!("ex: {:?}", ex);
    // println!("ey: {:?}", ey);
    let mut result = None;
    for i in (1..=(ex / steps.0 .0)).rev() {
        // println!("i: {}", i);
        if let Some(count) = min_count(steps, ex - steps.0 .0 * i, ey - steps.0 .1 * i, cache) {
            match result {
                None => {
                    result = Some(3 * i + count);
                }
                Some(number) => {
                    result = Some(min(number, 3 * i + count));
                }
            }
        }
    }
    for i in (1..=(ex / steps.1 .0)).rev() {
        if let Some(count) = min_count(steps, ex - steps.1 .0 * i, ey - steps.1 .1 * i, cache) {
            match result {
                None => {
                    result = Some(i + count);
                }
                Some(number) => {
                    result = Some(min(number, i + count));
                }
            }
        }
    }
    cache.insert((ex, ey), result);
    result
}

pub fn count(input: ((i64, i64), (i64, i64)), px: i64, py: i64) -> Option<i64> {
    let d = input.0 .0 * input.1 .1 - input.0 .1 * input.1 .0;
    let di = px * input.1 .1 - py * input.0 .1;
    let dj = input.0 .0 * py - input.1 .0 * px;
    if di % d == 0 && dj % d == 0 {
        return Some(3 * di / d + dj / d);
    } else {
        return None;
    }
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    for chunk in reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .chunks(4)
    {
        let (ax, ay) = get_numbers(&chunk[0]);
        let (bx, by) = get_numbers(&chunk[1]);
        let (ex, ey) = get_numbers(&chunk[2]);
        if let Some(count) = count(((ax, bx), (ay, by)), ex, ey) {
            // println!("Line: {}", chunk[2]);
            // println!("count: {}", count);
            result += count as i64;
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    for chunk in reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .chunks(4)
    {
        let (ax, ay) = get_numbers(&chunk[0]);
        let (bx, by) = get_numbers(&chunk[1]);
        let (ex, ey) = get_numbers(&chunk[2]);
        if let Some(count) = count(
            ((ax, bx), (ay, by)),
            ex + 10000000000000,
            ey + 10000000000000,
        ) {
            // println!("Line: {}", chunk[2]);
            // println!("count: {}", count);
            result += count as i64;
        }
    }
    result
}
