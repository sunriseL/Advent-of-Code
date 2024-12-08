use colored::*;
use std::any::type_name;
use std::path::Path;

mod advent_2024;

fn test_case<F>(func: F, input: &str, expect_result: Option<i64>)
where
    F: Fn(&str) -> i64,
{
    let name = type_name::<F>();
    let name = name[name.find("::").unwrap() + 2..].to_string();
    let result = func(input);
    if let Some(expect_result) = expect_result {
        if result == expect_result {
            println!("{}", name.on_green());
        } else {
            println!(
                "{}: {}(Expect) vs {}(Actual)",
                name.on_red(),
                expect_result,
                result
            );
        }
    } else {
        println!("{}: {}", name.on_white(), result);
    }
}

fn main() {
    let root = Path::new("..");

    let input_dir = root.join("input").join("2024");
    test_case(
        advent_2024::day1::part1,
        input_dir.join("day1").to_str().unwrap(),
        Some(2113135),
    );
    test_case(
        advent_2024::day1::part2,
        input_dir.join("day1").to_str().unwrap(),
        Some(19097157),
    );
    test_case(
        advent_2024::day2::part1,
        input_dir.join("day2").to_str().unwrap(),
        Some(242),
    );
    test_case(
        advent_2024::day2::part2,
        input_dir.join("day2").to_str().unwrap(),
        Some(311),
    );
    test_case(
        advent_2024::day3::part1,
        input_dir.join("day3").to_str().unwrap(),
        Some(173419328),
    );
    test_case(
        advent_2024::day3::part2,
        input_dir.join("day3").to_str().unwrap(),
        Some(90669332),
    );
    test_case(
        advent_2024::day4::part1,
        input_dir.join("day4").to_str().unwrap(),
        Some(2532),
    );
    test_case(
        advent_2024::day4::part2,
        input_dir.join("day4").to_str().unwrap(),
        Some(1941),
    );
    test_case(
        advent_2024::day5::part1,
        input_dir.join("day5").to_str().unwrap(),
        Some(4996),
    );
    test_case(
        advent_2024::day5::part2,
        input_dir.join("day5").to_str().unwrap(),
        Some(6311),
    );
    test_case(
        advent_2024::day6::part1,
        input_dir.join("day6").to_str().unwrap(),
        Some(4967),
    );
    test_case(
        advent_2024::day6::part2,
        input_dir.join("day6").to_str().unwrap(),
        Some(1789),
    );
}
