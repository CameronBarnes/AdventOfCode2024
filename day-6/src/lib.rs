pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
