pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<Robot>;

pub struct Robot {
    start_point: (i32, i32),
    velocity: (i32, i32),
}

pub fn run(part: Part) -> Output {
    let (input, x_size, y_size) = match part {
        Part::One | Part::Two => (input::read(), 101, 103),
        Part::ExampleOne | Part::ExampleTwo => (input::read_example(), 11, 7),
    };

    match part {
        Part::ExampleOne | Part::One => part1::solve(&input, x_size, y_size),
        Part::ExampleTwo | Part::Two => part2::solve(&input, x_size, y_size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 228690000);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        assert_eq!(result, 7093);
    }
}
