use crate::day06::{Direction, Input, Output};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Input) -> Output {
    let mut current_direction = Direction::Up;
    let mut current_x = input.start_point.x;
    let mut current_y = input.start_point.y;
    let mut visited_points = HashSet::from([(current_x, current_y)]);
    while let Some(point) = input.get_point_in_direction(current_x, current_y, &current_direction) {
        if (point.is_blockage) {
            current_direction = current_direction.rotate();
            continue;
        }
        visited_points.insert((point.x, point.y));
        current_x = point.x;
        current_y = point.y;
    }

    Output::U32(visited_points.len().try_into().unwrap())
}
