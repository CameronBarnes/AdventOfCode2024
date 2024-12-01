#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

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
