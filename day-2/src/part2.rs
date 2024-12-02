use itertools::Itertools;
use tinyvec::array_vec;

use crate::{is_safe, parse_num};

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let reports= input
        .lines()
        .map(|line| {
            let mut arr = array_vec!([u8; 8]);
            line.split_ascii_whitespace().map(parse_num).collect_into(&mut arr);
            arr
        })
        .collect_vec();
    reports
        .iter()
        .filter(|report| {
            if !is_safe(report.iter()) {
                (0..report.len()).any(|index| {
                    is_safe(
                        report
                            .iter()
                            .take(index)
                            .chain(report.iter().skip(index + 1)),
                    )
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

    #[test]
    fn test_real_input() {
        let input = include_str!("../input.txt");
        assert_eq!("476", process(input));
    }
}
