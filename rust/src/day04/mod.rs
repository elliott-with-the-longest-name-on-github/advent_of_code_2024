pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

// map of "x,y" => 'X'
pub type Input = Grid;

pub struct Grid {
    pub x_size: i32,
    pub y_size: i32,
    pub map: HashMap<String, char>,
}

pub enum Direction {
    Increase,
    Decrease,
    None,
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
        assert_eq!(result, 2545);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 1886);
    }
}
