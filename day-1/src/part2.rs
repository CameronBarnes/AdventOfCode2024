use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::parse_num;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    for (left_num, right_num) in input.lines().map(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        (parse_num(left), parse_num(right))
    }) {
        left.push(left_num);
        right.push(right_num);
    }

    let mut map: FxHashMap<usize, usize> = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
    let mut freq: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
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
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input));
    }
}
