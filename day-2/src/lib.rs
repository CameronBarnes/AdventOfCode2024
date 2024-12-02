#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(iter_collect_into)]
use itertools::Itertools;

pub mod part1;
pub mod part2;

#[inline]
pub fn parse_num(input: &str) -> u8 {
    let bytes = input.as_bytes();
    if bytes.len() == 1 {
        bytes[0] - b'0'
    } else {
        (bytes[0] - b'0') * 10 + bytes[1] - b'0'
    }
}

pub fn is_safe(report: impl Iterator<Item = &'_ u8> + std::clone::Clone) -> bool {
    (report.clone().tuple_windows().all(|(a, b)| a > b)
        || report.clone().tuple_windows().all(|(a, b)| a < b))
        && report
            .clone()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
}
