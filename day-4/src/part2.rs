use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let width = grid[0].len();
    grid.windows(3)
        .map(|window| {
            (0..(width - 2))
                // We do unsafe here to improve the unwrap performance, we know becase of the
                // checked and consistent width that this should never fail
                .filter(|col_index| unsafe {
                    let first: (&u8, &u8, &u8) = window[0]
                        .iter()
                        .skip(*col_index)
                        .take(3)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let second: (&u8, &u8, &u8) = window[1]
                        .iter()
                        .skip(*col_index)
                        .take(3)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let third: (&u8, &u8, &u8) = window[2]
                        .iter()
                        .skip(*col_index)
                        .take(3)
                        .collect_tuple()
                        .unwrap_unchecked();
                    matches!(
                        (first.0, second.1, third.2),
                        (b'M', b'A', b'S') | (b'S', b'A', b'M')
                    ) && matches!(
                        (first.2, second.1, third.0),
                        (b'M', b'A', b'S') | (b'S', b'A', b'M')
                    )
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
