pub fn numbers(input: &str) -> Vec<i64> {
    input
        .split(|c: char| !c.is_numeric() && c != '-')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}