pub mod part1;
pub mod part2;

pub fn is_safe(report: &[u32]) -> bool {
    (report.windows(2).all(|nums| nums[0] > nums[1])
        || report.windows(2).all(|nums| nums[0] < nums[1]))
        && report.windows(2).all(|nums| {
            let diff = nums[0].abs_diff(nums[1]);
            (1..=3).contains(&diff)
        })
}
