use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result: i64 = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let points : Vec<(i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        (x, y)
    }).collect();

    let mut max_area = i64::MIN;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let area = ((points[i].0 - points[j].0).abs() + 1) * ((points[i].1 - points[j].1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    result = max_area;
    result
}

fn cross(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
    // cross(b - a, c - a)
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn on_segment(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    // c 是否在 ab 线段上（假设三点共线）
    c.0 >= a.0.min(b.0)
        && c.0 <= a.0.max(b.0)
        && c.1 >= a.1.min(b.1)
        && c.1 <= a.1.max(b.1)
}

fn intersect(p1: (i64, i64), p2: (i64, i64), q1: (i64, i64), q2: (i64, i64)) -> bool {
    let o1 = cross(p1, p2, q1);
    let o2 = cross(p1, p2, q2);
    let o3 = cross(q1, q2, p1);
    let o4 = cross(q1, q2, p2);

    // 一般情况：跨过
    // if o1.signum() * o2.signum() < 0 && o3.signum() * o4.signum() < 0 {
    //     return true;
    // }

    // 特殊情况：共线且投影重叠
    // if o1 == 0 && on_segment(p1, p2, q1) { return true; }
    // if o2 == 0 && on_segment(p1, p2, q2) { return true; }
    // if o3 == 0 && on_segment(q1, q2, p1) { return true; }
    // if o4 == 0 && on_segment(q1, q2, p2) { return true; }
    // false

        // 必须严格一正一负，不能为 0
    // (o1 > 0 && o2 < 0 || o1 < 0 && o2 > 0) &&
    // (o3 > 0 && o4 < 0 || o3 < 0 && o4 > 0)

        // 先看是否有“广义相交”（包含端点、重叠）
    let general =
        (o1.signum() * o2.signum() <= 0) &&
        (o3.signum() * o4.signum() <= 0);

    if !general {
        return false;
    }

    // 如果不是全部共线，说明是正常相交或端点相交，直接算 true
    if !(o1 == 0 && o2 == 0 && o3 == 0 && o4 == 0) {
        return true;
    }

    // 剩下就是“全部共线”的情况：我们要区分
    //   - 只在端点接触（1 个点重合） → 允许
    //   - 有一段重叠 → 不算

    // 判断投影是否重叠的工具函数
    fn overlap_1d(a1: i64, a2: i64, b1: i64, b2: i64) -> i64 {
        let left = a1.min(a2).max(b1.min(b2));
        let right = a1.max(a2).min(b1.max(b2));
        (right - left).max(0)
    }

    let ox = overlap_1d(p1.0, p2.0, q1.0, q2.0);
    let oy = overlap_1d(p1.1, p2.1, q1.1, q2.1);

    // 如果在 x,y 上的重叠长度都是 0，说明最多只是“一个端点接触”
    // 一旦有正长度（>0），就是重叠一条线段，我们排除掉
    !(ox > 0 || oy > 0)

}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result:i64 = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let points : Vec<(i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        (x, y)
    }).collect();

    let mut max_area = i64::MIN;


    for i in 0..points.len() {
        for j in i+1..points.len() {
            let area = ((points[i].0 - points[j].0).abs() + 1) * ((points[i].1 - points[j].1).abs() + 1);
            if area > max_area {
                println!("i: {}, j: {}", i, j);
                println!("area: {}", area);
                let point1 = points[i];
                let point2 = points[j];
                let point3 = (point1.0, point2.1);
                let point4 = (point2.0, point1.1);

                let mut valid = true;
                for k in 0..points.len()-1 {
                    if k == i || k + 1 == i {
                        continue;
                    }
                    if intersect(point1, point3, points[k], points[k+1]) {
                        println!("point1: {:?}, point3: {:?}, seg: ({:?}, {:?})", point1, point3, points[k], points[k+1]);
                        valid = false;
                        break;
                    }
                    if intersect(point1, point4, points[k], points[k+1]) {
                        println!("point1: {:?}, point4: {:?}, seg: ({:?}, {:?})", point1, point4, points[k], points[k+1]);
                        valid = false;
                        break;
                    }
                }
                for k in 0..points.len()-1 {
                    if k == j || k + 1 == j {
                        continue;
                    }
                    if intersect(point2, point3, points[k], points[k+1]) {
                        println!("point2: {:?}, point3: {:?}, seg: ({:?}, {:?})", point2, point3, points[k], points[k+1]);
                        valid = false;
                        break;
                    }
                    if intersect(point2, point4, points[k], points[k+1]) {
                        println!("point2: {:?}, point4: {:?}, seg: ({:?}, {:?})", point2, point4, points[k], points[k+1]);
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
                max_area = area; 
            }
        }
    }
    result = max_area;
    result
}
