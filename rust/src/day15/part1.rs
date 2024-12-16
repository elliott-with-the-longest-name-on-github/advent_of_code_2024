use crate::day15::{Input, Output, WarehouseObjectType};

pub fn solve(input: &Input) -> Output {
    let mut robot_position = input.normal.robot_start;
    let mut objects = input.normal.objects.clone();
    for mv in input.normal.moves.iter() {
        let (mut x, mut y) = robot_position;
        let (x_change, y_change) = mv.to_xy();
        let mut box_pushed = false;
        loop {
            let next_x = x + x_change;
            let next_y = y + y_change;
            if let Some(object) = objects.get(&(next_x, next_y)) {
                match object {
                    WarehouseObjectType::Box => {
                        box_pushed = true;
                        x = next_x;
                        y = next_y;
                        continue;
                    }
                    WarehouseObjectType::Wall => break,
                };
            };

            // If we've made it here, we hit an empty spot before hitting a wall.
            // We need to update the robot position, remove the box from the robot's new position,
            // and add a box to the empty spot we've discovered _if_ we pushed a box.

            robot_position = (robot_position.0 + x_change, robot_position.1 + y_change);
            objects.remove(&robot_position);
            if (box_pushed) {
                objects.insert((next_x, next_y), WarehouseObjectType::Box);
            }
            break;
        }
    }

    objects
        .iter()
        .filter_map(|((x, y), obj_type)| {
            if (*obj_type != WarehouseObjectType::Box) {
                return None;
            }
            Some(y * 100 + x)
        })
        .sum::<i32>()
        .into()
}
