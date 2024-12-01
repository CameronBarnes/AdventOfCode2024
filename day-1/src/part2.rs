use rustc_hash::{FxBuildHasher, FxHashMap};
use tinyvec::array_vec;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut left = array_vec!([u32; 1024]);
    let mut right = array_vec!([u32; 1024]);
    let mut input = input.as_bytes();
    // While let matching based on code from @danielrab on discord
    while let [a0, a1, a2, a3, a4, _, _, _, b0, b1, b2, b3, b4, _, remaining_input @ ..] = input {
        input = remaining_input;
        left.push(
            (*a0 - b'0') as u32 * 10_000
                + (*a1 - b'0') as u32 * 1000
                + (*a2 - b'0') as u32 * 100
                + (*a3 - b'0') as u32 * 10
                + (*a4 - b'0') as u32,
        );
        right.push(
            (*b0 - b'0') as u32 * 10_000
                + (*b1 - b'0') as u32 * 1000
                + (*b2 - b'0') as u32 * 100
                + (*b3 - b'0') as u32 * 10
                + (*b4 - b'0') as u32,
        );
    }

    let mut map: FxHashMap<u32, u32> = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
    let mut freq: FxHashMap<u32, u32> = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
    left.iter().for_each(|num| {
        map.insert(*num, 0);
        freq.entry(*num).and_modify(|freq| *freq += 1).or_insert(1);
    });
    right.iter().for_each(|num| {
        map.entry(*num).and_modify(|val| *val += 1);
    });

    map.iter()
        .map(|(key, value)| {
            let freq = freq.get(key).unwrap_or(&0);
            *key * *value * freq
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("20520794", process(input));
    }
}
