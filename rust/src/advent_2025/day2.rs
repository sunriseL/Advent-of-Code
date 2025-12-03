use std::fs;

/// 解析输入格式: "start1-end1,start2-end2,..."
/// 返回一个 Vec<(i64, i64)>，每个元素是一个范围 (start, end)
fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    let content = fs::read_to_string(input).unwrap();
    content
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<i64>().ok()?;
                let end = parts[1].parse::<i64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    let mut result: i64 = 0;

    for range in ranges {
        let (start, end) = range;
        for i in start..=end {
            let digit = i.to_string().chars().count();
            if digit % 2 != 0 {
                continue;
            }
            let divider = 10_i64.pow((digit / 2) as u32);
            if (i % divider) == i / divider {
                result += i;
            }
        }
    }
    result
}

pub fn is_duplicate(i: i64, digit: i64, divide_count: i64) -> bool {
    if (digit % divide_count) != 0 {
        return false;
    }
    let divider = 10_i64.pow((digit / divide_count) as u32);
    let mut value = i;
    let mut first = None;
    while value > 0 {
        let rem = value % divider;
        if let Some(f) = first {
            if rem != f {
                return false;
            }
        } else {
            first = Some(rem);
        }
        value /= divider;
    }
    return true;
}

pub fn part2(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    let mut result: i64 = 0;
    for range in ranges {
        let (start, end) = range;
        for i in start..=end {
            let digit = i.to_string().chars().count();
            for divide_count in 2..=digit {
                if is_duplicate(i, digit as i64, divide_count as i64) {
                    result += i;
                    // println!("{}, divide_count: {}", i, divide_count);
                    break;
                }
            }
        }
    }
    result
}
