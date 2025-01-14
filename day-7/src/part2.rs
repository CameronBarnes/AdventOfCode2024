use itertools::Itertools;

fn solve(result: usize, mut arguments: Vec<usize>) -> bool {
    if arguments.len() == 1 {
        result == *arguments.first().unwrap()
    } else {
        let first = arguments.pop().unwrap();
        let second = arguments.pop().unwrap();
        if first > result || second > result {
            return false;
        }
        let mut product = arguments.clone();
        product.push(first * second);
        if !solve(result, product.clone()) {
            product.pop();
            product.push(first + second);
            if !solve(result, product.clone()) {
                product.pop();
                product.push(
                    (first.to_string() + &second.to_string())
                        .parse::<usize>()
                        .unwrap(),
                );
                solve(result, product)
            } else {
                true
            }
        } else {
            true
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let equations: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let (result, arguments) = line.split_once(": ").unwrap();
            let arguments = arguments
                .split_ascii_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .rev()
                .collect_vec();
            (result.parse().unwrap(), arguments)
        })
        .collect();

    equations
        .into_iter()
        .map(
            |(result, arguments)| {
                if solve(result, arguments) {
                    result
                } else {
                    0
                }
            },
        )
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input));
    }
}
