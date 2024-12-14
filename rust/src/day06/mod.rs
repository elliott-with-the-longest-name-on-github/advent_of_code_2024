pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

pub type Input = Grid;

#[derive(Clone)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
    pub is_blockage: bool,
}

pub struct Grid {
    pub grid: HashMap<(i32, i32), GridPoint>,
    pub start_point: GridPoint,
}

impl Grid {
    pub fn get_point_in_direction<'a>(
        &'a self,
        from_x: i32,
        from_y: i32,
        direction: &Direction,
    ) -> Option<&'a GridPoint> {
        match direction {
            Direction::Up => self.grid.get(&(from_x, from_y - 1)),
            Direction::Down => self.grid.get(&(from_x, from_y + 1)),
            Direction::Left => self.grid.get(&(from_x - 1, from_y)),
            Direction::Right => self.grid.get(&(from_x + 1, from_y)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
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
        assert_eq!(result, 6);
    }
}
