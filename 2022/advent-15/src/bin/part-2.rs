use advent_15::process_part_2;
use color_eyre::eyre;
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input.txt");
    println!("Answer: {}", process_part_2(input));
    Ok(())
}
