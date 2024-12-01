use ahash::{HashMap, HashMapExt};

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut freq: HashMap<usize, usize> = HashMap::new();
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
