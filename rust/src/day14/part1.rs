use std::collections::HashMap;

use crate::day14::{Input, Output};

pub fn solve(input: &Input, x_size: i32, y_size: i32) -> Output {
    let iterations = 100;
    let mut end_points = HashMap::new();
    for robot in input {
        let (x, y) = robot.start_point;
        let (vx, vy) = robot.velocity;
        let end_x = calculate_axis_end_position(x, vx, iterations, x_size);
        let end_y = calculate_axis_end_position(y, vy, iterations, y_size);
        *end_points.entry((end_x, end_y)).or_insert(0) += 1;
    }

    // this would break if the size of the arena wasn't odd, but it is, so I don't care
    let x_midpoint = (x_size - 1) / 2;
    let y_midpoint = (y_size - 1) / 2;
    let mut q1_total = 0;
    let mut q2_total = 0;
    let mut q3_total = 0;
    let mut q4_total = 0;
    for ((x, y), robot_count) in end_points.iter() {
        if (*x == x_midpoint || *y == y_midpoint) {
            continue;
        }

        match (*x < x_midpoint, *y < y_midpoint) {
            (true, true) => q1_total += robot_count,
            (false, true) => q2_total += robot_count,
            (true, false) => q3_total += robot_count,
            (false, false) => q4_total += robot_count,
        };
    }

    (q1_total * q2_total * q3_total * q4_total).into()
}

fn calculate_axis_end_position(start: i32, v: i32, iterations: i32, axis_size: i32) -> i32 {
    let virtual_position = (start + (v * iterations)) % axis_size;
    return (virtual_position + axis_size) % axis_size;
}
