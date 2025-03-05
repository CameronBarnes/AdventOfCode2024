use crate::checksum;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut storage: Vec<u16> = Vec::with_capacity(input.len() * 2);
    let mut empty = false;
    let mut index = 0;
    input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .map(|c| (c - b'0') as usize)
        .for_each(|num| {
            if empty {
                if num != 0 {
                    storage.append(&mut vec![u16::MAX; num]);
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

fn degrag(storage: &mut [u16]) {
    let mut left_index = 0;
    let mut right_index = storage.len() - 1;
    while left_index < right_index {
        if storage[left_index] == u16::MAX {
            while right_index > left_index {
                if storage[right_index] != u16::MAX {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("6332189866718", process(input))
    }

    #[test]
    fn test_1() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input));
    }
}
