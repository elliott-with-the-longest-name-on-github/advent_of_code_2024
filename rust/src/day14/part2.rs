use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    thread,
    time::Duration,
};

use crate::{
    day14::{Input, Output},
    util::Framebuffer,
};

pub fn solve(input: &Input, x_size: i32, y_size: i32) -> Output {
    let mut answers = Vec::new();
    let mut framebuffer = Framebuffer::new(10_000);
    for i in 0..10_000 {
        let mut end_points = HashMap::new();
        for robot in input {
            let (x, y) = robot.start_point;
            let (vx, vy) = robot.velocity;
            let end_x = calculate_axis_end_position(x, vx, i, x_size);
            let end_y = calculate_axis_end_position(y, vy, i, y_size);
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

        // My first time I actually did it visually

        let mut debug = String::new();
        for y in 0..y_size {
            for x in 0..x_size {
                let num = end_points
                    .get(&(x, y))
                    .map_or_else(|| String::from("."), |v| v.to_string());
                debug.push_str(&num);
            }
            debug.push('\n');
        }

        framebuffer.push(debug);
        framebuffer.debug();

        answers.push((i, q1_total * q2_total * q3_total * q4_total));
        thread::sleep(Duration::from_millis(20));
    }
    answers.sort_by(|a, b| a.1.cmp(&b.1));
    answers.first().unwrap().0.into()
}

fn calculate_axis_end_position(start: i32, v: i32, iterations: i32, axis_size: i32) -> i32 {
    let virtual_position = (start + (v * iterations)) % axis_size;
    return (virtual_position + axis_size) % axis_size;
}
