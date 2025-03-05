use itertools::Itertools;

use crate::checksum;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut storage: Vec<(u16, u16)> = Vec::with_capacity(input.len());
    let mut empty = false;
    let mut index = 0;
    input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .map(|c| (c - b'0') as u16)
        .for_each(|num| {
            if empty {
                if num != 0 {
                    storage.push((u16::MAX, num));
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
    degrag(&mut storage);
    // println!("{}", super::dbg_storage(&convert(&storage)));

    checksum(&convert(&storage)).to_string()
}

fn degrag(storage: &mut Vec<(u16, u16)>) {
    let mut empty_end_indexes = 0;
    let mut start_index_by_needed_size: [usize; 10] = [0; 10];
    let file_ids = storage
        .iter()
        .rev()
        .filter(|(id, _size)| *id != u16::MAX)
        .map(|(id, _size)| *id)
        .collect_vec();
    for file_id in file_ids {
        if let Some((index, (file_id, size))) = storage
            .iter()
            .enumerate()
            .rev()
            .skip(empty_end_indexes)
            .find_map(|(index, (id, size))| {
                if *id == file_id {
                    Some((index, (*id, *size)))
                } else {
                    None
                }
            })
        {
            let start = start_index_by_needed_size[size as usize];
            if let Some((empty_index, empty)) = storage
                .iter()
                .enumerate()
                .skip(start)
                .filter(|(empty_index, (file_id, _size))| {
                    *empty_index < index && *file_id == u16::MAX
                })
                .find(|(_, (_id, empty_size))| *empty_size >= size)
            {
                // println!("Empty slot, index: {empty_index} value: {empty:?}");
                // println!("Full slot, index: {index} value: {file_id}, {size}");
                let diff = empty.1 - size;
                if diff == 0 {
                    // println!("Swapping {index} and {empty_index}");
                    start_index_by_needed_size[size as usize] = start.max(empty_index);
                    storage.swap(index, empty_index);
                    empty_end_indexes = storage.len() - index;
                } else {
                    // println!("Moving from {index} to {empty_index} with {diff} left over");
                    start_index_by_needed_size[size as usize] = start.max(empty_index);
                    storage.get_mut(index).unwrap().0 = u16::MAX;
                    *storage.get_mut(empty_index).unwrap() = (file_id, size);
                    storage.insert(empty_index + 1, (u16::MAX, diff));
                    empty_end_indexes = storage.len() - (index + 1);
                }
            }
            // println!("{}", super::dbg_storage(&convert(storage)));
        } else {
            break;
        }
    }
}

fn convert(storage: &[(u16, u16)]) -> Vec<u16> {
    storage
        .iter()
        .flat_map(|(index, size)| vec![*index; *size as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("6353648390778", process(input))
    }

    #[test]
    fn test_1() {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input));
    }

    #[test]
    fn test_2() {
        let input = "1313165";
        assert_eq!("169", process(input));
    }
}
