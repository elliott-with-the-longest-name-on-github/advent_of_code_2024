pub mod input;
pub mod part1;
pub mod part2;

use std::collections::{HashMap, HashSet};

use crate::{Output, Part};

pub type Input = PrintConfig;

pub struct PrintConfig {
    pub rules: HashMap<u32, HashSet<u32>>,
    pub updates: Vec<Vec<u32>>,
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
        assert_eq!(result, 4766);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 6257);
    }
}
