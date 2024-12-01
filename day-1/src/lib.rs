pub mod part1;
pub mod part2;

fn parse_num(input: &str) -> usize {
    let mut out = 0;
    for (index, c) in input.bytes().enumerate().rev() {
        out += (c - 48) as usize * (index * 10);
    }
    out
}
