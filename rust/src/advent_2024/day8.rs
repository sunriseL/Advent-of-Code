use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut map = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut row_len: usize = 0;
    let mut col_len: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        row_len = i;
        let line = line.unwrap();
        col_len = line.len();
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                map.entry(c)
                    .or_insert(Vec::<(i32, i32)>::new())
                    .push((i as i32, j as i32))
            }
        });
    }
    let mut antennas = HashSet::<(i32, i32)>::new();
    row_len += 1;
    for (c, points) in map {
        for point1 in points.as_slice() {
            for point2 in points.as_slice() {
                if point1 == point2 {
                    continue;
                }
                // println!("search {:?} vs {:?}", point1, point2);
                let new_point = (2 * point1.0 - point2.0, 2 * point1.1 - point2.1);
                if (0..row_len as i32).contains(&new_point.0)
                    && (0..col_len as i32).contains(&new_point.1)
                {
                    antennas.insert(new_point);
                }
            }
        }
    }
    // println!("antennas: {:?}", antennas);
    antennas.len() as i64
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut map = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut row_len: usize = 0;
    let mut col_len: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        row_len = i;
        let line = line.unwrap();
        col_len = line.len();
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                map.entry(c)
                    .or_insert(Vec::<(i32, i32)>::new())
                    .push((i as i32, j as i32))
            }
        });
    }
    let mut antennas = HashSet::<(i32, i32)>::new();
    row_len += 1;
    for (c, points) in map {
        for point1 in points.as_slice() {
            for point2 in points.as_slice() {
                if point1 == point2 {
                    continue;
                }
                // println!("search {:?} vs {:?}", point1, point2);
                let delta = (point2.0 - point1.0, point2.1 - point1.1);
                let mut new_point = point2.clone();
                loop {
                    // println!("new_point: {:?}", new_point);
                    if (0..row_len as i32).contains(&new_point.0)
                        && (0..col_len as i32).contains(&new_point.1)
                    {
                        antennas.insert(new_point);
                    } else {
                        break;
                    }
                    new_point = (new_point.0 + delta.0, new_point.1 + delta.1);
                }
            }
        }
    }
    // println!("antennas: {:?}", antennas);
    antennas.len() as i64
}
