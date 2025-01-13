use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use itertools::Itertools;

fn validate(pages: &[u16], key: &HashMap<u16, Vec<u16>>) -> bool {
    let mut set: HashSet<u16> = HashSet::new();
    for page in pages {
        set.insert(*page);
        if let Some(list) = key.get(page) {
            for required in list {
                if pages.contains(required) && !set.contains(required) {
                    // println!("Invalid: {pages:?}");
                    // println!("Missing {required} before {page}\n");
                    return false;
                }
            }
        }
    }
    true
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut iter = input.lines();
    let mut map: HashMap<u16, Vec<u16>> = HashMap::new();
    for line in iter.by_ref() {
        if let Some((a, b)) = line.split_once("|") {
            map.entry(b.parse().unwrap())
                .and_modify(|vec| vec.push(a.parse().unwrap()))
                .or_insert(vec![a.parse().unwrap()]);
        } else {
            break;
        }
    }

    let mut sum = 0;
    for line in iter {
        if !line.is_empty() {
            let nums = line
                .split(",")
                .map(|num| num.parse::<u16>().unwrap())
                .collect_vec();
            if validate(&nums, &map) {
                let middle = nums[nums.len() / 2];
                // println!("Valid: {nums:?} Middle: {middle}\n");
                sum += middle;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input));
    }
}
