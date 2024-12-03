use crate::parse_str;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut first = input.split("don't()");
    let mut sum = parse_str(first.next().unwrap());
    first.for_each(|dont| {
        dont.split("do()").skip(1).for_each(|allowed| {
            sum += parse_str(allowed);
        })
    });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input));
    }
}
