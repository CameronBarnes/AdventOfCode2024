use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let horizontal = grid
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|window| {
                    (window[0] == b'X'
                        && window[1] == b'M'
                        && window[2] == b'A'
                        && window[3] == b'S')
                        || (window[0] == b'S'
                            && window[1] == b'A'
                            && window[2] == b'M'
                            && window[3] == b'X')
                })
                .count()
        })
        .sum::<usize>();
    let width = grid[0].len();
    let vertical = (0..width)
        .map(|x_pos| {
            let col = grid
                .iter()
                .map(|line| line.get(x_pos).unwrap())
                .collect_vec();
            col.windows(4)
                .filter(|window| {
                    (*window[0] == b'X'
                        && *window[1] == b'M'
                        && *window[2] == b'A'
                        && *window[3] == b'S')
                        || (*window[0] == b'S'
                            && *window[1] == b'A'
                            && *window[2] == b'M'
                            && *window[3] == b'X')
                })
                .count()
        })
        .sum::<usize>();
    let height = grid.len();
    let left_right_diagonal = (0..(height - 3))
        .map(|row| {
            if row == 0 {
                // Handle the first row
                (0..width)
                    .map(|offset| {
                        let diag = grid
                            .iter()
                            .enumerate()
                            .filter_map(|(row_index, row)| row.get(row_index + offset))
                            .collect_vec();
                        diag.windows(4)
                            .filter(|window| {
                                (*window[0] == b'X'
                                    && *window[1] == b'M'
                                    && *window[2] == b'A'
                                    && *window[3] == b'S')
                                    || (*window[0] == b'S'
                                        && *window[1] == b'A'
                                        && *window[2] == b'M'
                                        && *window[3] == b'X')
                            })
                            .count()
                    })
                    .sum()
            } else {
                let diag = grid
                    .iter()
                    .skip(row)
                    .enumerate()
                    .filter_map(|(row_index, row)| row.get(row_index))
                    .collect_vec();
                diag.windows(4)
                    .filter(|window| {
                        (*window[0] == b'X'
                            && *window[1] == b'M'
                            && *window[2] == b'A'
                            && *window[3] == b'S')
                            || (*window[0] == b'S'
                                && *window[1] == b'A'
                                && *window[2] == b'M'
                                && *window[3] == b'X')
                    })
                    .count()
            }
        })
        .sum::<usize>();
    let right_left_diagonal = (0..(height - 3))
        .map(|row| {
            if row == 0 {
                // Handle the first row
                (0..width)
                    .map(|offset| {
                        let diag = grid
                            .iter()
                            .rev()
                            .enumerate()
                            .filter_map(|(row_index, row)| row.get(row_index + offset))
                            .collect_vec();
                        diag.windows(4)
                            .filter(|window| {
                                (*window[0] == b'X'
                                    && *window[1] == b'M'
                                    && *window[2] == b'A'
                                    && *window[3] == b'S')
                                    || (*window[0] == b'S'
                                        && *window[1] == b'A'
                                        && *window[2] == b'M'
                                        && *window[3] == b'X')
                            })
                            .count()
                    })
                    .sum()
            } else {
                let diag = grid
                    .iter()
                    .rev()
                    .skip(row)
                    .enumerate()
                    .filter_map(|(row_index, row)| row.get(row_index))
                    .collect_vec();
                diag.windows(4)
                    .filter(|window| {
                        (*window[0] == b'X'
                            && *window[1] == b'M'
                            && *window[2] == b'A'
                            && *window[3] == b'S')
                            || (*window[0] == b'S'
                                && *window[1] == b'A'
                                && *window[2] == b'M'
                                && *window[3] == b'X')
                    })
                    .count()
            }
        })
        .sum::<usize>();
    let total = horizontal + vertical + left_right_diagonal + right_left_diagonal;
    total.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        "4"
    )]
    #[case(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        "18"
    )]
    #[case(
        "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX",
        "18"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
