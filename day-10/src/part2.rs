use cached::proc_macro::cached;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Load the input into a vec
    let grid = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| c - b'0').collect_vec())
        .collect_vec();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, val)| **val == 0)
                .map(|(x, _)| check(x, y, u8::MAX, &grid))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn check(x: usize, y: usize, prev: u8, grid: &[Vec<u8>]) -> usize {
    if prev == u8::MAX || grid[y][x].wrapping_sub(1) == prev {
        count_path(x, y, grid)
    } else {
        0
    }
}

#[cached(key = "(usize, usize)", convert = "{ (x, y) }")]
fn count_path(x: usize, y: usize, grid: &[Vec<u8>]) -> usize {
    let current = grid[y][x];
    if current == 9 {
        1
    } else {
        let mut count = 0;
        if x > 0 {
            count += check(x - 1, y, current, grid);
        }
        if y > 0 {
            count += check(x, y - 1, current, grid);
        }
        if y < grid.len() - 1 {
            count += check(x, y + 1, current, grid);
        }
        if x < grid[0].len() - 1 {
            count += check(x + 1, y, current, grid);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input));
    }

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("1942", process(input));
    }
}
