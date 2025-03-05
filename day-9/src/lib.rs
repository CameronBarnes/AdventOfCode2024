pub mod part1;
pub mod part2;

pub fn dbg_storage(storage: &[u16]) -> String {
    storage
        .iter()
        .map(|num| match *num {
            u16::MAX => ".".to_string(),
            num => num.to_string(),
        })
        .collect()
}

pub fn checksum(storage: &[u16]) -> usize {
    storage
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if *value == u16::MAX {
                0
            } else {
                index * (*value as usize)
            }
        })
        .sum()
}
