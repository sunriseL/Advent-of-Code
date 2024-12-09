use std::fs::File;
use std::io::{BufReader, Read};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut v =  Vec::<Option<i32>>::new();
    for (i, c) in reader.bytes().enumerate() {
        if let Some(c) = c.ok() {
            let number = c - b'0';
            if i % 2 == 1 {
                for _i in 0..number {
                    v.push(None);
                }
            } else {
                for _i in 0..number {
                    v.push(Some(i as i32 / 2));
                }
            }
        }
    }
    let mut i: usize = 0;
    let mut j: usize = v.len() - 1;
    loop {
        // println!("check {} {}", i, j);
        if i == j {
            break;
        }
        if v[i].is_some() {
            i += 1;
            continue;
        }
        if v[j].is_none() {
            j -= 1;
            continue;
        }
        // println!("swap {} {}", i, j);
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
    for (i, element) in v.iter().enumerate() {
        if let Some(number) = element {
            result += (number * i as i32) as i64;
        }
    }
    // println!("v: {:?}", v);
    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let mut files = Vec::<(usize, i64, i64)>::new();
    let mut blanks = Vec::<(usize, i64)>::new();
    let mut len: usize = 0;
    for (i, c) in reader.bytes().enumerate() {
        if let Some(c) = c.ok() {
            let number = c - b'0';
            if i % 2 == 0 {
                files.push((len, number as i64, i as i64 / 2));
            } else {
                blanks.push((len, number as i64));
            }
            len += number as usize;
        }
    }
    // println!("{:?}", files);
    // println!("{:?}", blanks);
    let mut results= Vec::<(usize, i64, i64)>::new();
    for file in files.iter().rev() {
        let (index, len, value) = file;
        if let Some(blank) = blanks.iter_mut().find(|(_index, _len)| _len >= len && _index < index) {
            results.push((blank.0, *len, *value));
            blank.0 += *len as usize;
            blank.1 -= len;
        } else {
            results.push(*file);
        }
    }
    // println!("{:?}", results);
    for (index, len, value) in results.iter() {
        result += ((*index as i64..*index as i64+len).sum::<i64>() * value) as i64;
    }
    result
}