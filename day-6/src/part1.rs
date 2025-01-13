use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }

    fn apply_movement(
        &self,
        mut across: usize,
        mut vertical: usize,
        bounds: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if vertical == 0 {
                    return None;
                }
                vertical -= 1;
            }
            Direction::Right => {
                if across == bounds - 1 {
                    return None;
                }
                across += 1;
            }
            Direction::Down => {
                if vertical == bounds - 1 {
                    return None;
                }
                vertical += 1;
            }
            Direction::Left => {
                if across == 0 {
                    return None;
                }
                across -= 1;
            }
        }
        Some((across, vertical))
    }
}

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
