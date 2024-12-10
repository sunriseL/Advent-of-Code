use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn search_point(
    mat: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    expect_number: i32,
    results: &mut HashSet<(usize, usize)>,
) {
    if row >= mat.len() || col >= mat[0].len() {
        return;
    }
    if results.contains(&(row, col)) {
        return;
    }
    if mat[row][col] != expect_number {
        return;
    }
    if expect_number == 9 {
        results.insert((row, col));
    } else {
        search_point(mat, row.wrapping_add(1), col, expect_number + 1, results);
        search_point(mat, row, col.wrapping_add(1), expect_number + 1, results);
        search_point(mat, row.wrapping_sub(1), col, expect_number + 1, results);
        search_point(mat, row, col.wrapping_sub(1), expect_number + 1, results);
    }
}

fn search_path(
    mat: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    expect_number: i32,
    result: &mut i64,
) {
    if row >= mat.len() || col >= mat[0].len() {
        return;
    }
    if mat[row][col] != expect_number {
        return;
    }
    if expect_number == 9 {
        *result += 1;
    } else {
        search_path(mat, row.wrapping_add(1), col, expect_number + 1, result);
        search_path(mat, row, col.wrapping_add(1), expect_number + 1, result);
        search_path(mat, row.wrapping_sub(1), col, expect_number + 1, result);
        search_path(mat, row, col.wrapping_sub(1), expect_number + 1, result);
    }
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut map = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut mat = Vec::<Vec<i32>>::new();
    let mut result: i64 = 0;
    let mut row_len: usize = 0;
    let mut col_len: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        mat.push(line.bytes().map(|c| (c - b'0') as i32).collect());
    }
    for row in 0..mat.len() {
        for col in 0..mat[row].len() {
            let mut set = HashSet::<(usize, usize)>::new();
            search_point(&mat, row, col, 0, &mut set);
            result += set.len() as i64;
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut map = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut mat = Vec::<Vec<i32>>::new();
    let mut result: i64 = 0;
    let mut row_len: usize = 0;
    let mut col_len: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        mat.push(line.bytes().map(|c| (c - b'0') as i32).collect());
    }
    for row in 0..mat.len() {
        for col in 0..mat[row].len() {
            let mut set = HashSet::<(usize, usize)>::new();
            search_path(&mat, row, col, 0, &mut result);
        }
    }
    result
}