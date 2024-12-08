use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // up
    (0, 1), // right
    (1, 0), // down
    (0, -1), // left
];

const DIRECTIONS_WEIGHT: [((i32, i32), i32); 4] = [
    ((-1, 0), 1), // up
    ((0, 1), 2), // right
    ((1, 0), 4), // down
    ((0, -1), 8), // left
];

fn step(pos: (i32, i32), direction: &(i32, i32)) -> (i32, i32) {
    (pos.0 + direction.0, pos.1 + direction.1)
}

fn get(map: &Vec<Vec<i32>>, pos: (i32, i32)) -> Option<i32> {
    if let Some(row) = map.get(pos.0 as usize) {
        if let Some(ele) = row.get(pos.1 as usize) {
            return Some(*ele);
        }
    }
    None
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().map(|c| if c == '#' { 2 } else { 0 }).collect());
        // Find start point
        if let Some(pos) = line.chars().position(|x| x == '^') {
            start = (map.len() as i32 - 1, pos as i32);
        }
    }
    let mut pos = start;
    let mut direction_iter= DIRECTIONS.iter().cycle();
    let mut direction = direction_iter.next().take().expect("");
    loop {
        match get(&map, pos) {
            Some(number) => {
                if number == 0 {
                    map[pos.0 as usize][pos.1 as usize] = 1;
                    result += 1;
                    pos = step(pos, &direction);
                } else if number == 1 {
                    pos = step(pos, &direction);
                } else if number == 2 {
                    pos = step(pos, &(-direction.0, -direction.1));
                    direction = direction_iter.next().take().expect("");
                }

            }
            None => {
                break;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().map(|c| if c == '#' { 16 } else { 0 }).collect());
        // Find start point
        if let Some(pos) = line.chars().position(|x| x == '^') {
            start = (map.len() as i32 - 1, pos as i32);
        }
    }
    for i in 0..map.len() {
        let row = map.get(i).unwrap();
        for j in 0..row.len() {
            let ele = get(&map, (i as i32, j as i32)).unwrap();
            if ele == 2 || (i as i32, j as i32) == start {
                continue;
            } else {
                let mut _map = map.clone();
                _map[i][j] = 16;
                let mut pos = start;
                let mut direction_iter= DIRECTIONS_WEIGHT.iter().cycle();
                let (mut direction, mut weight) = direction_iter.next().take().expect("");
                let mut is_loop = false;
                loop {
                    match get(&_map, pos) {
                        Some(number) => {
                            if number < 16 {
                                _map[pos.0 as usize][pos.1 as usize] |= weight;
                                pos = step(pos, &direction);
                                if number & weight > 0 {
                                    is_loop = true;
                                    break;
                                }
                            } else {
                                pos = step(pos, &(-direction.0, -direction.1));
                                let direction_weight = direction_iter.next().take().expect("");
                                direction = direction_weight.0;
                                weight = direction_weight.1;
                                continue;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                result += is_loop as i64;
            }

        }
    }
    result
}