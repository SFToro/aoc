fn main() {
    let input: &str = include_str!("../../part-two.txt");

    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    todo!("Implement process");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 142);
    }
}
