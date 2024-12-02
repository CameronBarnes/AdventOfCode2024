use itertools::Itertools;

use crate::is_safe;

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
            if !is_safe(report) {
                (0..report.len()).any(|index| {
                    let mut changed = (*report).clone();
                    changed.remove(index);
                    is_safe(&changed)
                })
            } else {
                true
            }
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
        assert_eq!("4", process(input));
    }
}
