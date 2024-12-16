pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

pub type Input = Warehouses;

pub struct Warehouses {
    normal: Warehouse,
    doublewide: DoublewideWarehouse,
}

pub struct Warehouse {
    robot_start: (i32, i32),
    objects: HashMap<(i32, i32), WarehouseObjectType>,
    moves: Vec<Direction>,
}

pub struct DoublewideWarehouse {
    robot_start: (i32, i32),
    objects: HashMap<(i32, i32), DoublewideWarehouseObjectType>,
    moves: Vec<Direction>,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_xy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    fn reverse(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    fn endcap(direction: &Direction) -> Side {
        match direction {
            Direction::Left => Side::Left,
            Direction::Right => Side::Right,
            _ => panic!("Tried to endcap a vertical direction."),
        }
    }

    // Returns the x diff (1 or -1) needed to match the other side
    fn match_side(&self) -> i32 {
        match self {
            Side::Left => 1,
            Side::Right => -1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum WarehouseObjectType {
    Box,
    Wall,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DoublewideWarehouseObjectType {
    Box(Side),
    Wall,
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
        assert_eq!(result, 1577255);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::ExampleTwo);
        assert_eq!(result, 1597035);
    }
}
