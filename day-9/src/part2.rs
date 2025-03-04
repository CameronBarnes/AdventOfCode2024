use itertools::Itertools;

use crate::checksum;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut storage: Vec<(i32, usize)> = Vec::new();
    let mut empty = false;
    let mut index = 0;
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Failed with char {c}")) as usize
        })
        .for_each(|num| {
            if empty {
                if num != 0 {
                    storage.push((-1, num));
                }
                empty = false;
            } else {
                if num != 0 {
                    storage.push((index, num));
                }
                empty = true;
                index += 1;
            }
        });

    // println!("{}", super::dbg_storage(&convert(&storage)));
    // The bellow code was a silly attempt to maximize compression
    // let mut moved = 1;
    // while moved > 0 {
    //     moved = degrag(&mut storage);
    //     println!("Moved: {moved}");
    // }
    degrag(&mut storage);
    // println!("{}", super::dbg_storage(&convert(&storage)));

    checksum(&convert(&storage)).to_string()
}

fn degrag(storage: &mut Vec<(i32, usize)>) -> usize {
    let mut moves = 0;
    let file_ids = storage
        .iter()
        .rev()
        .filter(|(id, _size)| *id >= 0)
        .map(|(id, _size)| *id)
        .collect_vec();
    for file_id in file_ids {
        if let Some((index, (file_id, size))) =
            storage.iter().enumerate().find_map(|(index, (id, size))| {
                if *id == file_id {
                    Some((index, (*id, *size)))
                } else {
                    None
                }
            })
        {
            if let Some((empty_index, empty)) = storage
                .iter()
                .enumerate()
                .filter(|(empty_index, (file_id, _size))| *empty_index < index && *file_id < 0)
                .find(|(_, (_id, empty_size))| *empty_size >= size)
            {
                // println!("Empty slot, index: {empty_index} value: {empty:?}");
                // println!("Full slot, index: {index} value: {file_id}, {size}");
                let diff = empty.1 - size;
                if diff == 0 {
                    // println!("Swapping {index} and {empty_index}");
                    storage.swap(index, empty_index);
                } else {
                    // println!("Moving from {index} to {empty_index} with {diff} left over");
                    storage.get_mut(index).unwrap().0 = -1;
                    *storage.get_mut(empty_index).unwrap() = (file_id, size);
                    storage.insert(empty_index + 1, (-1, diff));
                }
                moves += 1;
            }
            // println!("{}", super::dbg_storage(&convert(storage)));
        } else {
            break;
        }
    }
    moves
}

fn convert(storage: &[(i32, usize)]) -> Vec<i32> {
    storage
        .iter()
        .flat_map(|(index, size)| vec![*index; *size])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input));
    }

    #[test]
    fn test_2() {
        let input = "1313165";
        assert_eq!("169", process(input));
    }
}
