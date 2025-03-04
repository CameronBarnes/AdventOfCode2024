pub mod part1;
pub mod part2;

pub fn dbg_storage(storage: &[i32]) -> String {
    storage
        .iter()
        .map(|num| match num {
            -1 => ".".to_string(),
            num => num.to_string(),
        })
        .collect()
}

pub fn checksum(storage: &[i32]) -> usize {
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
