use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // top-left
    ( 0, -1), // top
    ( 1, -1), // top-right
    (-1,  0), // left
    ( 1,  0), // right
    (-1,  1), // bottom-left
    ( 0,  1), // bottom
    ( 1,  1), // bottom-right
];

fn is_element(matrix: &Vec<Vec<char>>, r: i32, c: i32, e: char) -> Result<(), Box<dyn Error>> {
    let row = matrix.get(usize::try_from(r)?);
    let element = row.ok_or("Not exist".to_string())?.get(usize::try_from(c)?);
    if *element.ok_or("Not exist".to_string())? == e {
        Ok(())
    } else {
        Err("Miss match".into())
    } 
}

fn valid_count(matrix: &Vec<Vec<char>>, r: i32, c: i32) -> Result<i64, Box<dyn Error>> {
    is_element(matrix, r, c, 'X')?;
    let mut result: i64 = 0;
    for direction in DIRECTIONS {
        if is_element(matrix, r + direction.0, c + direction.1, 'M').is_err() {
            continue;
        }
        if is_element(matrix, r + 2 * direction.0, c + 2 * direction.1, 'A').is_err() {
            continue;
        }
        if is_element(matrix, r + 3 * direction.0, c + 3 * direction.1, 'S').is_err() {
            continue;
        }
        result += 1;
    }
    Ok(result)
}

fn valid_xmas(matrix: &Vec<Vec<char>>, r: i32, c: i32) -> Result<i64, Box<dyn Error>> {
    is_element(matrix, r, c, 'A')?;
    let mut result: i64 = 0;
    for (e1, e2) in [('S', 'M'), ('M', 'S')] {
        for is_horizontal in [false, true] {
            if is_horizontal {
                if is_element(matrix, r - 1, c - 1, e1).is_err() {
                    continue;
                }
                if is_element(matrix, r - 1, c + 1, e1).is_err() {
                    continue;
                }
                if is_element(matrix, r + 1, c - 1, e2).is_err() {
                    continue;
                }
                if is_element(matrix, r + 1, c + 1, e2).is_err() {
                    continue;
                }
                return Ok(1)
            } else {
                if is_element(matrix, r - 1, c - 1, e1).is_err() {
                    continue;
                }
                if is_element(matrix, r + 1, c - 1, e1).is_err() {
                    continue;
                }
                if is_element(matrix, r - 1, c + 1, e2).is_err() {
                    continue;
                }
                if is_element(matrix, r + 1, c + 1, e2).is_err() {
                    continue;
                }
                return Ok(1)
            }
        }
    }
    Ok(0)
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut matrix = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        matrix.push(line.unwrap().chars().collect());
    }
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if let Ok(number) = valid_count(&matrix, i32::try_from(r).ok().unwrap(), i32::try_from(c).ok().unwrap()) {
                result += number;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut matrix = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        matrix.push(line.unwrap().chars().collect());
    }
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if let Ok(number) = valid_xmas(&matrix, i32::try_from(r).ok().unwrap(), i32::try_from(c).ok().unwrap()) {
                result += number;
            }
        }
    }
    result
}
