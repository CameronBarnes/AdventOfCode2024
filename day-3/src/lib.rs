use std::sync::LazyLock;

use regex::Regex;

pub mod part1;
pub mod part2;

pub fn parse_str(input: &str) -> u32 {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    RE.captures_iter(input)
        .map(|capture| capture.extract())
        .map(|(_, [num1, num2])| {
            num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}
