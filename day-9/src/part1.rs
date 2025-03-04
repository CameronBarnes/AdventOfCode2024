#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut storage: Vec<i32> = Vec::new();
    let mut empty = false;
    let mut index = 0;
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Failed with char {c}")) as usize)
        .for_each(|num| {
            if empty {
                if num != 0 {
                    storage.append(&mut vec![-1; num]);
                }
                empty = false;
            } else {
                if num != 0 {
                    storage.append(&mut vec![index; num]);
                }
                empty = true;
                index += 1;
            }
        });

    degrag(&mut storage);

    checksum(&storage).to_string()
}

#[allow(dead_code)]
fn dbg_storage(storage: &[i32]) -> String {
    storage
        .iter()
        .map(|num| match num {
            -1 => ".".to_string(),
            num => num.to_string(),
        })
        .collect()
}

fn degrag(storage: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = storage.len() - 1;
    while left_index < right_index {
        if storage[left_index] == -1 {
            while right_index > left_index {
                if storage[right_index] != -1 {
                    storage.swap(left_index, right_index);
                    break;
                } else {
                    right_index -= 1;
                }
            }
        } else {
            left_index += 1;
        }
    }
}

fn checksum(storage: &[i32]) -> usize {
    storage
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if *value < 0 {
                0
            } else {
                index * (*value as usize)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input));
    }
}
