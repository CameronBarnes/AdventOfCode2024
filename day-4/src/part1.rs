use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // For use with real input only
    let grid = unsafe { input.as_bytes().as_chunks_unchecked::<141>() };
    //let grid = input.lines().map(|line| line.as_bytes()).collect_vec(); // For use with test input
    let width = grid[0].len();
    let diagonal = grid
        .windows(4)
        .map(|window| {
            (0..(width - 3))
                // We do unsafe here to improve the unwrap performance, we know becase of the
                // checked and consistent width that this should never fail
                .map(|col_index| unsafe {
                    let first: (&u8, &u8, &u8, &u8) = window[0]
                        .iter()
                        .skip(col_index)
                        .take(4)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let second: (&u8, &u8, &u8, &u8) = window[1]
                        .iter()
                        .skip(col_index)
                        .take(4)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let third: (&u8, &u8, &u8, &u8) = window[2]
                        .iter()
                        .skip(col_index)
                        .take(4)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let forth: (&u8, &u8, &u8, &u8) = window[3]
                        .iter()
                        .skip(col_index)
                        .take(4)
                        .collect_tuple()
                        .unwrap_unchecked();
                    let mut matches = 0;
                    if matches!(
                        (first.0, second.1, third.2, forth.3),
                        (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')
                    ) {
                        // Left-right diagonal
                        matches += 1;
                    }
                    if matches!(
                        (first.3, second.2, third.1, forth.0),
                        (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')
                    ) {
                        // Right-left diagonal
                        matches += 1;
                    }
                    if matches!(
                        (first.0, second.0, third.0, forth.0),
                        (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')
                    ) {
                        // Left edge vertical
                        matches += 1;
                    }
                    if matches!(first, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X')) {
                        // Top edge horizontal
                        matches += 1;
                    }
                    matches
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    let horizontal = grid
        .iter()
        .skip(grid.len() - 3)
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
    let vertical = ((width - 3)..width)
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
    let total = diagonal + horizontal + vertical;
    total.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[ignore] // Swap out the parsing to use lines and collect vec to use this test
    #[case(
        "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        "4"
    )]
    #[ignore] // Swap out the parsing to use lines and collect vec to use this test
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
    #[ignore] // Swap out the parsing to use lines and collect vec to use this test
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
    #[ignore] // Swap out the parsing to use lines and collect vec to use this test
    #[case(
        "S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S",
        "8"
    )]
    #[ignore] // Swap out the parsing to use lines and collect vec to use this test
    #[case(
        "XMASAMX
MM...MM
A.A.A.A
S..S..S
A.A.A.A
MM...MM
XMASAMX",
        "12"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!("2567", process(input));
    }
}
