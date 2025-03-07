use cached::proc_macro::cached;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    private_process(input, 25)
}

fn private_process(input: &str, iterations: usize) -> String {
    input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .map(|stone| count_stones(stone, iterations))
        .sum::<usize>()
        .to_string()
}

#[cached]
fn count_stones(stone: u64, blinks: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 1 10 99 999", "7", 1)]
    #[case("125 17", "22", 6)]
    #[case("125 17", "55312", 25)]
    #[case(include_str!("../input.txt"), "203953", 25)]
    fn test_1(#[case] input: &str, #[case] expected_result: &str, #[case] iterations: usize) {
        assert_eq!(expected_result, private_process(input, iterations));
    }
}
