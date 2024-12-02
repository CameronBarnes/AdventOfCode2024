#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(iter_collect_into)]
use itertools::Itertools;

pub mod part1;
pub mod part2;

pub fn is_safe(report: impl Iterator<Item = &'_ u8> + std::clone::Clone) -> bool {
    (report.clone().tuple_windows().all(|(a, b)| a > b)
        || report.clone().tuple_windows().all(|(a, b)| a < b))
        && report
            .clone()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
}
