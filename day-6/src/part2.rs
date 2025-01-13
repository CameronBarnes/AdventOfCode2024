use ahash::{HashSet, HashSetExt};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Direction;

fn solve_looping(map: &[Vec<u8>], mut vert: usize, mut across: usize, bounds: usize) -> bool {
    let mut direction = Direction::Up;
    let mut state: HashSet<(Direction, usize, usize)> = HashSet::new();
    while let Some((horizontal, vertical)) = direction.apply_movement(across, vert, bounds) {
        if map[vertical][horizontal] == 1 {
            direction = direction.right();
        } else {
            vert = vertical;
            across = horizontal;
            if !state.insert((direction, vertical, horizontal)) {
                return true;
            }
        }
    }
    false
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0u8,
                    '#' => 1u8,
                    '^' => 2u8,
                    _ => unreachable!("There should only be 3 values present"),
                })
                .collect_vec()
        })
        .collect_vec();
    let bounds = map.len();
    let mut vert = map
        .iter()
        .find_position(|line| line.contains(&2))
        .unwrap()
        .0;
    let start_vert = vert;
    let mut across = map[vert]
        .iter()
        .find_position(|char| **char == 2)
        .unwrap()
        .0;
    let start_across = across;
    let mut direction = Direction::Up;

    let mut state: HashSet<(usize, usize)> = HashSet::new();
    while let Some((horizontal, vertical)) = direction.apply_movement(across, vert, bounds) {
        if map[vertical][horizontal] == 1 {
            direction = direction.right();
        } else {
            vert = vertical;
            across = horizontal;
            state.insert((vertical, horizontal));
        }
    }

    state
        .par_iter()
        .filter(|(change_vert, change_across)| {
            let mut map = map.clone();
            map[*change_vert][*change_across] = 1;
            solve_looping(&map, start_vert, start_across, bounds)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input));
    }
}
