use crate::day06::{Direction, Input, Output};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Input) -> Output {
    let mut current_direction = Direction::Up;
    let mut current_point = input.start_point.clone();
    let mut visited_cols = HashSet::from([(current_direction.clone(), current_point.x)]);
    let mut visited_rows = HashSet::from([(current_direction.clone(), current_point.y)]);
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

        let direction_visited = match next_direction {
            Direction::Down | Direction::Up => {
                visited_cols.contains(&(next_direction.clone(), current_point.x))
            }
            Direction::Left | Direction::Right => {
                visited_cols.contains(&(next_direction.clone(), current_point.y))
            }
        };

        if (direction_visited) {
            // The current point is an intersection with a line we've already travelled, in a direction we've already travelled.
            // If there are no obstructions before we reach a point we've already touched, this is a loop.
            // We can start theoretically traveling in that direction until we hit a block or a point we've already travelled.
            let mut theoretical_travel_point = current_point.clone();
            loop {
                if (visited_points.contains(&(
                    next_direction.clone(),
                    theoretical_travel_point.x,
                    theoretical_travel_point.y,
                ))) {
                    additional_blockages.insert((point.x, point.y));
                    break;
                }
                if (theoretical_travel_point.is_blockage) {
                    break;
                }
                theoretical_travel_point = match input.get_point_in_direction(
                    theoretical_travel_point.x,
                    theoretical_travel_point.y,
                    &next_direction,
                ) {
                    Some(p) => p.clone(),
                    None => break,
                }
            }
        };

        visited_cols.insert((current_direction.clone(), current_point.x));
        visited_rows.insert((current_direction.clone(), current_point.y));
        visited_points.insert((current_direction.clone(), current_point.x, current_point.y));
        current_point = point.clone();
    }

    Output::U32(additional_blockages.len().try_into().unwrap())
}
