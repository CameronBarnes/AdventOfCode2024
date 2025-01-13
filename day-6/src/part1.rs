use itertools::Itertools;

use super::Direction;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut map = input
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
    let mut across = map[vert]
        .iter()
        .find_position(|char| **char == 2)
        .unwrap()
        .0;
    let mut direction = Direction::Up;

    while let Some((horizontal, vertical)) = direction.apply_movement(across, vert, bounds) {
        if map[vertical][horizontal] == 1 {
            direction = direction.right();
        } else {
            vert = vertical;
            across = horizontal;
            map[vertical][horizontal] = 3;
        }
    }

    // Count result
    map.iter()
        .flatten()
        .filter(|c| **c == 3)
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
        assert_eq!("41", process(input));
    }
}
