#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    let mut input = input.as_bytes();
    // While let matching based on code from @danielrab on discord
    while let [a0, a1, a2, a3, a4, _, _, _, b0, b1, b2, b3, b4, _, remaining_input @ ..] = input {
        input = remaining_input;
        left.push(
            (*a0 - b'0') as u32 * 10_000
                + (*a1 - b'0') as u32 * 1000
                + (*a2 - b'0') as u32 * 100
                + (*a3 - b'0') as u32 * 10
                + (*a4 - b'0') as u32,
        );
        right.push(
            (*b0 - b'0') as u32 * 10_000
                + (*b1 - b'0') as u32 * 1000
                + (*b2 - b'0') as u32 * 100
                + (*b3 - b'0') as u32 * 10
                + (*b4 - b'0') as u32,
        );
    }

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.max(right) - left.min(right))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("1765812", process(input));
    }
}
