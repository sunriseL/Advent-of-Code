use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};



fn fill_zone(mat: &mut Vec<Vec<Option<char>>>, i: usize, j: usize, zone: &mut HashSet::<(usize, usize)>, expect: &char) {
    if let Some(row) = mat.get(i) {
        if let Some(c) = row.get(j) {
            if let Some(c) = c {
                if c == expect {
                    zone.insert((i, j));
                    mat[i][j] = None;
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        if let (Some(x), Some(y)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
                            fill_zone(mat, x, y, zone, expect);
                        }
                    }
                }
            }
        }
    }
}

fn len(zone: &HashSet::<(usize, usize)>) -> i64 {
    let mut result: i64 = 0;
    let mut shared_count: i64 = 0;
    for (i, j) in zone {
        result += 4;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let (Some(x), Some(y)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
                shared_count += zone.contains(&(x, y)) as i64;
            }
        }
    }
    result - shared_count
}

fn sides(zone: &HashSet::<(usize, usize)>) -> i64 {
    let mut shared_count: i64 = 0;
    let mut lines =  HashSet::<(usize, usize, usize)>::new();
    for (i, j) in zone {
        for (direction, dx, dy) in [(0, -1, 0), (1, 1, 0), (2, 0, -1), (3, 0, 1)] {
            let mut has_neighbour = false;
            if let (Some(x), Some(y)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
                has_neighbour = zone.contains(&(x,y));
            }
            if !has_neighbour {
                lines.insert((direction, *i, *j));
            }
        }
    }
    for (direction, x, y) in lines.iter() {
        if (2..4).contains(direction) {
            for dx in [-1, 1] {
                if let Some(x) = x.checked_add_signed(dx) {
                    shared_count += lines.contains(&(*direction, x, *y)) as i64;
                }
            }
        } else {
            for dy in [-1, 1] {
                if let Some(y) = y.checked_add_signed(dy) {
                    shared_count += lines.contains(&(*direction, *x, y)) as i64;
                }
            }
        }
    }
    lines.len() as i64 - shared_count / 2
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut mat = Vec::<Vec<Option<char>>>::new();
    let mut zones = Vec::<HashSet<(usize, usize)>>::new();
    let mut result: i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        mat.push(line.chars().map(|c| Some(c)).collect());
    }
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if let Some(c) = mat[i][j] {
                let mut zone = HashSet::<(usize, usize)>::new();
                fill_zone(&mut mat, i, j, &mut zone, &c);
                zones.push(zone);
            }
        }
    }
    for zone in zones.iter() {
        result += zone.len() as i64 * len(&zone);
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut mat = Vec::<Vec<Option<char>>>::new();
    let mut zones = Vec::<HashSet<(usize, usize)>>::new();
    let mut result: i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        mat.push(line.chars().map(|c| Some(c)).collect());
    }
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if let Some(c) = mat[i][j] {
                let mut zone = HashSet::<(usize, usize)>::new();
                fill_zone(&mut mat, i, j, &mut zone, &c);
                zones.push(zone);
            }
        }
    }
    for zone in zones.iter() {
        result += zone.len() as i64 * sides(&zone);
    }
    result
}