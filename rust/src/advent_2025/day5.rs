use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut pairs: Vec<(i64, i64)> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();
    let mut parsing_pairs = true;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            parsing_pairs = false;
            continue;
        }

        if parsing_pairs {
            // Parse pair like "5086738007739-6875107477983"
            let parts: Vec<&str> = line.split('-').collect();
            let first: i64 = parts[0].parse().unwrap();
            let second: i64 = parts[1].parse().unwrap();
            pairs.push((first, second));
        } else {
            // Parse single number
            let num: i64 = line.parse().unwrap();
            numbers.push(num);
        }
    }

    (pairs, numbers)
}

pub fn part1(input: &str) -> i64 {
    let (pairs, numbers) = parse_input(input);
    let mut result: i64 = 0;

    for number in numbers {
        let mut valid = false;
        for pair in &pairs {
            if number >= pair.0 && number <= pair.1 {
                valid = true;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    result
}

pub fn merge_intervals(intervals: &mut Vec<(i64, i64)>, pair: (i64, i64)) {
    let mut merged_pair = pair;
    let mut i = 0;
    let changed = false;
    while i < intervals.len() {
        let a = intervals[i].0;
        let b = intervals[i].1;
        // overlap if max(a, min) <= min(b, max)
        if merged_pair.1 >= a && merged_pair.0 <= b {
            // merge
            merged_pair.0 = merged_pair.0.min(a);
            merged_pair.1 = merged_pair.1.max(b);
            // erase interval
            intervals.remove(i);

            i = 0;
            continue;
            // do not increment i, as the vector is now shorter
        } else {
            i += 1;
        }
    }
    intervals.push(merged_pair);
}

pub fn part2(input: &str) -> i64 {
    let (pairs, numbers) = parse_input(input);
    let mut result: i64 = 0;
    let mut copy_pairs = pairs.clone();
    let mut intervals: Vec<(i64, i64)> = Vec::new();
    for pair in &pairs {
        merge_intervals(&mut intervals, *pair);
    }
    for interval in intervals {
        result += interval.1 - interval.0 + 1;
    }

    result
}
