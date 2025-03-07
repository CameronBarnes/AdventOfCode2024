use std::collections::LinkedList;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    private_process(input, 25)
}

fn private_process(input: &str, iterations: usize) -> String {
    let mut stones: LinkedList<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    for _ in 0..iterations {
        let mut cursor = stones.cursor_front_mut();
        while let Some(current) = cursor.current().cloned() {
            if current == 0 {
                *cursor.current().unwrap() = 1;
                cursor.move_next();
            } else if (current.ilog10() + 1) & 1 == 0 {
                let split_at = current.ilog10().div_ceil(2);
                let str = current.to_string();
                let (first, second) = str.split_at(split_at as usize);
                *cursor.current().unwrap() = first.parse().unwrap();
                cursor.insert_after(second.parse().unwrap());
                cursor.move_next();
                cursor.move_next();
            } else {
                *cursor.current().unwrap() *= 2024;
                cursor.move_next();
            }
        }
    }

    stones.len().to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 1 10 99 999", "7", 1)]
    #[case("125 17", "22", 6)]
    #[case("125 17", "55312", 25)]
    fn test_1(#[case] input: &str, #[case] expected_result: &str, #[case] iterations: usize) {
        assert_eq!(expected_result, private_process(input, iterations));
    }
}
