use std::sync::LazyLock;

use regex::Regex;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(mul)\((\d+),(\d+)\)").unwrap());

#[tracing::instrument]
pub fn process(input: &str) -> String {
    RE.captures_iter(input)
        .map(|capture| capture.extract())
        .map(|(_, [_action, num1, num2])| {
            num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input));
    }
}
