use std::fs::File;
use std::io::{BufRead, BufReader};
use super::util::numbers;

fn pos(robot: (i64, i64, i64, i64), secs: i64, wid: i64, len: i64) -> (i64, i64) {
    let mut x = robot.0;
    let mut y = robot.1;
    x += secs * robot.2;
    while x < 0 {
        x += wid;
    }
    while x >= wid {
        x -= wid;
    }
    y += secs * robot.3;
    while y < 0 {
        y += len;
    }
    while y >= len{
        y -= len;
    }
    (x, y)
}

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut v = Vec::<(i64, i64, i64,i64)>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers = numbers(&line);
        v.push((numbers[0], numbers[1], numbers[2], numbers[3]));
    }
    let wid: i64 = 101;
    let len: i64 = 103;
    let mut a1: i64 = 0;
    let mut a2: i64 = 0;
    let mut a3: i64 = 0;
    let mut a4: i64 = 0;
    for robot in v.iter() {
        // println!("robot: {:?}", robot);
        let (x, y) = pos(*robot, 100, wid, len);
        // println!("{},{}", x, y);
        if x - wid / 2 < 0 {
            if y - len / 2 < 0 {
                a3 += 1;
            } else if y - len / 2 > 0 {
                a2 += 1
            }
        } else if x - wid / 2 > 0 {
            if y - len / 2 < 0 {
                a4 += 1;
            } else if y - len / 2 > 0 {
                a1 += 1
            }
        }
    }
    // println!("{} {} {} {}",a1,a2,a3,a4);
    a1 * a2 * a3 * a4
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut v = Vec::<(i64, i64, i64,i64)>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers = numbers(&line);
        v.push((numbers[0], numbers[1], numbers[2], numbers[3]));
    }
    const wid: i64 = 101;
    const len: i64 = 103;

    let mut step = 83;
    for i in 0..200 {
        let mut m: [[i64; wid as usize]; len as usize] = [[0; wid as usize]; len as usize];
        for robot in v.iter() {
            let (x, y) = pos(*robot, step + i * 101, wid, len);
            m[y as usize][x as usize] += 1;
        }
        // println!("Steps: {}", step + i * 101);
        // for row in m.iter() {
        //     println!("{}", row.iter()
        //     .map(|&num| if num == 0 { ".".to_string() } else {format!("{:0>1}", num) })
        //     .collect::<Vec<String>>()
        //     .concat());
        // }
    }
    6446 // Uncomment the code and you will find the answer
}