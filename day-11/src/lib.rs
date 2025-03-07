use cached::proc_macro::cached;

pub mod part1;
pub mod part2;

pub fn process(input: &str, iterations: usize) -> String {
    input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .map(|stone| count_stones(stone, iterations))
        .sum::<usize>()
        .to_string()
}

#[cached]
pub fn count_stones(stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }
    if stone == 0 {
        count_stones(1, blinks - 1)
    } else if (stone.ilog10() + 1) & 1 == 0 {
        let split_at = stone.ilog10().div_ceil(2);
        let str = stone.to_string();
        let (first, second) = str.split_at(split_at as usize);
        count_stones(first.parse().unwrap(), blinks - 1)
            + count_stones(second.parse().unwrap(), blinks - 1)
    } else {
        count_stones(stone * 2024, blinks - 1)
    }
}
