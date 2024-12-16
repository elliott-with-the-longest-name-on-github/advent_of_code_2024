use std::collections::{HashMap, HashSet};

use crate::day15::{Direction, DoublewideWarehouseObjectType, Input, Output, Side};

pub fn solve(input: &Input) -> Output {
    let mut robot_position = input.doublewide.robot_start;
    let mut objects = input.doublewide.objects.clone();
    for mv in input.doublewide.moves.iter() {
        match mv {
            Direction::Left | Direction::Right => {
                move_horizontal(&mut objects, &mut robot_position, &mv)
            }
            Direction::Up | Direction::Down => {
                move_vertical(&mut objects, &mut robot_position, &mv)
            }
        }
    }

    objects
        .iter()
        .filter_map(|((x, y), obj_type)| match obj_type {
            DoublewideWarehouseObjectType::Box(Side::Left) => Some(y * 100 + x),
            _ => None,
        })
        .sum::<i32>()
        .into()
}

fn move_horizontal(
    objects: &mut HashMap<(i32, i32), DoublewideWarehouseObjectType>,
    robot_position: &mut (i32, i32),
    direction: &Direction,
) {
    let (mut x, mut y) = robot_position;
    let (x_change, _) = direction.to_xy(); // we don't need Y, it won't ever change moving horizontally
    let new_robot_x = x + x_change;
    let mut move_ops = Vec::new();

    loop {
        let next_x = x + x_change;
        if let Some(object) = objects.get(&(next_x, y)) {
            match object {
                DoublewideWarehouseObjectType::Box(side) => {
                    let reversed = side.reverse();
                    move_ops.push((next_x, y, reversed));
                    x = next_x;
                    continue;
                }
                DoublewideWarehouseObjectType::Wall => break,
            };
        };

        // If we've made it here, we hit an empty spot before hitting a wall.
        // We need to update the robot position, remove the box from the robot's new position,
        // and add a box to the empty spot we've discovered _if_ we pushed a box.
        // Unlike in part 1, it's not as simple as just removing a box and adding one at the end --
        // we have to actually move all of the boxes in the line, otherwise we'll end up with mismatched box sides.

        if (!move_ops.is_empty()) {
            objects.insert(
                (next_x, y),
                DoublewideWarehouseObjectType::Box(Side::endcap(direction)),
            );
        }
        for (x, y, side) in move_ops {
            objects.insert((x, y), DoublewideWarehouseObjectType::Box(side));
        }

        *robot_position = (new_robot_x, robot_position.1);
        objects.remove(&robot_position);
        break;
    }
}

fn move_vertical(
    objects: &mut HashMap<(i32, i32), DoublewideWarehouseObjectType>,
    robot_position: &mut (i32, i32),
    direction: &Direction,
) {
    let (mut x, mut y) = robot_position;
    let (_, y_change) = direction.to_xy(); // we don't need X, it won't ever change moving vertically
    let new_robot_y = y + y_change;
    let mut affected_x = HashSet::from([x]);
    let mut move_ops = Vec::new();

    // We're going for readability here while sacrificing a little bit of perf
    loop {
        let next_y = y + y_change;
        if affected_x
            .iter()
            .all(|x| !objects.contains_key(&(*x, next_y)))
        {
            // If we've made it here, we hit an empty section before hitting a wall.
            // We need to update the robot position and translate all touched boxes one row up or down.
            // There's probably some optimizations we could make for no-op moves (where a box takes the place of a box that's in the same place)
            // but that doesn't seem worth it

            // these have to be reversed so that we don't overwrite other boxes
            // (think: move the boxes into the open space, which creates an open space
            // for the next boxes)
            for (x, y) in move_ops.iter().rev() {
                let Some(b) = objects.remove(&(*x, *y)) else {
                    panic!("This should be very impossible");
                };
                match b {
                    DoublewideWarehouseObjectType::Box(_) => {
                        objects.insert((*x, y + y_change), b);
                    }
                    _ => panic!("This should also be impossible"),
                };
            }
            *robot_position = (robot_position.0, new_robot_y);
            break;
        }

        // If we've made it here, we're blocked by boxes or walls
        if affected_x.iter().any(|x| match objects.get(&(*x, next_y)) {
            Some(obj) => *obj == DoublewideWarehouseObjectType::Wall,
            None => false,
        }) {
            // Wall, cannot move
            break;
        }

        // No walls, all boxes
        let just_boxes = affected_x
            .iter()
            .filter_map(|x| match objects.get(&(*x, next_y)) {
                Some(obj) => match obj {
                    DoublewideWarehouseObjectType::Box(side) => Some((*x, x + side.match_side())),
                    DoublewideWarehouseObjectType::Wall => None,
                },
                None => None,
            })
            .fold(HashSet::new(), |mut acc, (x1, x2)| {
                acc.insert(x1);
                acc.insert(x2);
                acc
            });
        for box_x in just_boxes.iter() {
            move_ops.push((*box_x, next_y))
        }
        affected_x = just_boxes;
        y = next_y;
    }
}
