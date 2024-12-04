use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let width = grid[0].len();
    grid.windows(3)
        .map(|window| {
            (0..width)
                .filter(|col_index| {
                    let first = window[0].iter().skip(*col_index).take(3).collect_vec();
                    let second = window[1].iter().skip(*col_index).take(3).collect_vec();
                    let third = window[2].iter().skip(*col_index).take(3).collect_vec();
                    if first.len() == 3 && second.len() == 3 && third.len() == 3 {
                        matches!(
                            (first[0], second[1], third[2]),
                            (b'M', b'A', b'S') | (b'S', b'A', b'M')
                        ) && matches!(
                            (first[2], second[1], third[0]),
                            (b'M', b'A', b'S') | (b'S', b'A', b'M')
                        )
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "M.S
.A.
M.S",
        "1"
    )]
    #[case(
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        "9"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
