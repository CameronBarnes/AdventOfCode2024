use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    reports
        .iter()
        .filter(|report| {
            (report.windows(2).all(|nums| nums[0] > nums[1])
                || report.windows(2).all(|nums| nums[0] < nums[1]))
                && report.windows(2).all(|nums| {
                    let diff = nums[0].abs_diff(nums[1]);
                    (1..=3).contains(&diff)
                })
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input));
    }
}
