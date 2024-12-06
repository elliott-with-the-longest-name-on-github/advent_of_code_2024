use crate::day06::{Input, Output};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &Input) -> Output {
    let mut visited_points = HashSet::new();
    let mut direction = Direction::Up;
    let mut current_x = input.start_x;
    let mut current_y = input.start_y;
    loop {
        let (new_direction, new_x, new_y, stop) = travel_until_blockage(
            &input.blockages_by_x,
            &input.blockages_by_y,
            &direction,
            &current_x,
            &current_y,
            &input.x_len,
            &input.y_len,
        );
        for y in min(current_y, new_y)..=max(current_y, new_y) {
            for x in min(current_x, new_x)..=max(current_x, new_x) {
                visited_points.insert((x, y));
            }
        }
        direction = new_direction;
        current_x = new_x;
        current_y = new_y;

        if (stop) {
            break;
        }
    }

    Output::U32(visited_points.len().try_into().unwrap())
}

fn travel_until_blockage(
    blockages_by_x: &HashMap<u32, Vec<u32>>,
    blockages_by_y: &HashMap<u32, Vec<u32>>,
    direction: &Direction,
    current_x: &u32,
    current_y: &u32,
    bound_x: &u32,
    bound_y: &u32,
) -> (Direction, u32, u32, bool) {
    let mut stop = false;
    let (blockages_in_axis, applicable_coordinate, last_traversable_coordinate) = match direction {
        Direction::Up => (blockages_by_x.get(current_x), current_y, 0),
        Direction::Down => (blockages_by_x.get(current_x), current_y, bound_x - 1),
        Direction::Left => (blockages_by_y.get(current_y), current_x, 0),
        Direction::Right => (blockages_by_y.get(current_y), current_x, bound_y - 1),
    };
    let stop_coord = match direction {
        Direction::Left | Direction::Up => match blockages_in_axis
            .and_then(|b| b.iter().rev().find(|coord| *coord < applicable_coordinate))
        {
            Some(blockage_coord) => blockage_coord + 1,
            None => {
                stop = true;
                last_traversable_coordinate
            }
        },
        Direction::Right | Direction::Down => match blockages_in_axis
            .and_then(|b| b.iter().find(|coord| *coord > applicable_coordinate))
        {
            Some(blockage_coord) => blockage_coord - 1,
            None => {
                stop = true;
                last_traversable_coordinate
            }
        },
    };
    let (stop_x, stop_y) = match direction {
        Direction::Up | Direction::Down => (*current_x, stop_coord),
        Direction::Left | Direction::Right => (stop_coord, *current_y),
    };

    (rotate_direction(direction), stop_x, stop_y, stop)
}

fn rotate_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
