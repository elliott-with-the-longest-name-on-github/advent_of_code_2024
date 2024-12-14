use crate::day06::{Direction, Input, Output};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Input) -> Output {
    let mut current_direction = Direction::Up;
    let mut current_point = input.start_point.clone();
    let mut visited_points =
        HashSet::from([(current_direction.clone(), current_point.x, current_point.y)]);
    let mut additional_blockages = HashSet::new();
    while let Some(point) =
        input.get_point_in_direction(current_point.x, current_point.y, &current_direction)
    {
        let next_direction = current_direction.rotate();

        if (point.is_blockage) {
            current_direction = next_direction;
            continue;
        }

        let mut theoretical_travel_point = point.clone();
        let mut theoretical_travel_direction = next_direction.clone();
        loop {
            let next_point = match input.get_point_in_direction(
                theoretical_travel_point.x,
                theoretical_travel_point.y,
                &theoretical_travel_direction,
            ) {
                Some(p) => p.clone(),
                None => break,
            };

            if (next_point.is_blockage) {
                theoretical_travel_direction = theoretical_travel_direction.rotate();
                continue;
            }

            if (visited_points.contains(&(
                theoretical_travel_direction.clone(),
                next_point.x,
                next_point.y,
            ))) {
                additional_blockages.insert((point.x, point.y));
                break;
            }

            theoretical_travel_point = next_point;
        }

        visited_points.insert((current_direction.clone(), point.x, point.y));
        current_point = point.clone();
    }

    Output::U32(additional_blockages.len().try_into().unwrap())
}
