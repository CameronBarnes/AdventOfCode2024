#[tracing::instrument]
pub fn process(input: &str) -> String {
    super::process(input, 25)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("0 1 10 99 999", "7", 1)]
    #[case("125 17", "22", 6)]
    #[case("125 17", "55312", 25)]
    #[case(include_str!("../input.txt"), "203953", 25)]
    fn test_1(#[case] input: &str, #[case] expected_result: &str, #[case] iterations: usize) {
        assert_eq!(expected_result, crate::process(input, iterations));
    }
}
