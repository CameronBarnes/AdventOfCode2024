use crate::parse_num;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    for (left_num, right_num) in input.lines().map(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        (parse_num(left), parse_num(right))
    }) {
        left.push(left_num);
        right.push(right_num);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.max(right) - left.min(right))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input));
    }
}
