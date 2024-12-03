use day_3::part1::process;

#[tracing::instrument]
fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);
}
