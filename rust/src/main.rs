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
        advent_2024::day1_part1::process,
        input_dir.join("day1").to_str().unwrap(),
        Some(2113135),
    );
    test_case(
        advent_2024::day1_part2::process,
        input_dir.join("day1").to_str().unwrap(),
        Some(19097157),
    );
    test_case(
        advent_2024::day2_part1::process,
        input_dir.join("day2").to_str().unwrap(),
        Some(242),
    );
    test_case(
        advent_2024::day2_part2::process,
        input_dir.join("day2").to_str().unwrap(),
        Some(311),
    );
    test_case(
        advent_2024::day3_part1::process,
        input_dir.join("day3").to_str().unwrap(),
        Some(173419328),
    );
    test_case(
        advent_2024::day3_part2::process,
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
}
