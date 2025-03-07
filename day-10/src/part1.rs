use ahash::{HashSet, HashSetExt};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Load the input into a vec
    let grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| if *c == b'.' { u8::MAX } else { c - b'0' })
                .collect_vec()
        })
        .collect_vec();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, val)| **val == 0)
                .map(|(x, _)| check(x, y, u8::MAX, &grid, &mut HashSet::new()))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn check(
    x: usize,
    y: usize,
    prev: u8,
    grid: &[Vec<u8>],
    checked: &mut HashSet<(usize, usize)>,
) -> usize {
    if prev == u8::MAX || grid[y][x].wrapping_sub(1) == prev {
        if checked.contains(&(x, y)) {
            0
        } else {
            checked.insert((x, y));
            count_path(x, y, grid, checked)
        }
    } else {
        0
    }
}

fn count_path(
    x: usize,
    y: usize,
    grid: &[Vec<u8>],
    checked: &mut HashSet<(usize, usize)>,
) -> usize {
    let current = grid[y][x];
    if current == 9 {
        1
    } else {
        let mut count = 0;
        if x > 0 {
            count += check(x - 1, y, current, grid, checked);
        }
        if y > 0 {
            count += check(x, y - 1, current, grid, checked);
        }
        if y < grid.len() - 1 {
            count += check(x, y + 1, current, grid, checked);
        }
        if x < grid[0].len() - 1 {
            count += check(x + 1, y, current, grid, checked);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "36"
    )]
    #[case(
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        "2"
    )]
    #[case(
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        "4"
    )]
    #[case(
        "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        "3"
    )]
    fn test(#[case] input: &str, #[case] result: &str) {
        assert_eq!(result, process(input));
    }

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("796", process(input));
    }
}
