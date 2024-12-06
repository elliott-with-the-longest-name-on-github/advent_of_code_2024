pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

pub type Input = ParsedMap;

pub struct ParsedMap {
    blockages_by_x: HashMap<u32, Vec<u32>>,
    blockages_by_y: HashMap<u32, Vec<u32>>,
    start_x: u32,
    start_y: u32,
    x_len: u32,
    y_len: u32,
}

pub fn run(part: Part) -> Output {
    let input = match part {
        Part::One | Part::Two => input::read(),
        Part::ExampleOne | Part::ExampleTwo => input::read_example(),
    };

    match part {
        Part::ExampleOne | Part::One => part1::solve(&input),
        Part::ExampleTwo | Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 5531);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::ExampleTwo);
        assert_eq!(result, 0);
    }
}
