#[tracing::instrument]
pub fn process(input: &str) -> String {
    super::process(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("242090118578155", process(input));
    }
}
