use std::collections::HashMap;

use crate::day15::{
    Direction, DoublewideWarehouse, DoublewideWarehouseObjectType, Input, Side, Warehouse,
    WarehouseObjectType, Warehouses,
};

const EXAMPLE_INPUT: &str = include_str!("../../input/15/example.txt");
const INPUT: &str = include_str!("../../input/15/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();

    let (normal_robot_start, normal_warehouse) = parse_warehouse_objects(warehouse);
    let (doublewide_robot_start, doublewide_warehouse) =
        parse_doublewide_warehouse_objects(warehouse);

    let moves: Vec<Direction> = moves
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        })
        .collect();

    return Warehouses {
        normal: Warehouse {
            robot_start: normal_robot_start,
            objects: normal_warehouse,
            moves: moves.clone(),
        },
        doublewide: DoublewideWarehouse {
            robot_start: doublewide_robot_start,
            objects: doublewide_warehouse,
            moves,
        },
    };
}

fn parse_warehouse_objects(raw: &str) -> ((i32, i32), HashMap<(i32, i32), WarehouseObjectType>) {
    let mut robot_start = None;
    let mut warehouse_objects = HashMap::new();
    for (y, line) in raw.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let object_type = match c {
                '#' => Some(WarehouseObjectType::Wall),
                '@' => {
                    robot_start = Some((x as i32, y as i32));
                    None
                }
                'O' => Some(WarehouseObjectType::Box),
                _ => None,
            };

            if let Some(object_type) = object_type {
                warehouse_objects.insert((x as i32, y as i32), object_type);
            }
        }
    }

    (robot_start.unwrap(), warehouse_objects)
}

fn parse_doublewide_warehouse_objects(
    raw: &str,
) -> (
    (i32, i32),
    HashMap<(i32, i32), DoublewideWarehouseObjectType>,
) {
    let mut robot_start = None;
    let mut warehouse_objects = HashMap::new();

    let widended_raw: String = raw
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            _ => "\n", // this is a bug if there's any character other than a '\n', but there isn't
        })
        .collect();

    for (y, line) in widended_raw.lines().enumerate() {
        let char_iter = line.chars().enumerate();
        for (x, c) in char_iter {
            let object_type = match c {
                '#' => Some(DoublewideWarehouseObjectType::Wall),
                '@' => {
                    robot_start = Some((x as i32, y as i32));
                    None
                }
                '[' => Some(DoublewideWarehouseObjectType::Box(Side::Left)),
                ']' => Some(DoublewideWarehouseObjectType::Box(Side::Right)),
                _ => None,
            };

            if let Some(object_type) = object_type {
                warehouse_objects.insert((x as i32, y as i32), object_type);
            }
        }
    }

    (robot_start.unwrap(), warehouse_objects)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        assert_eq!(input.normal.robot_start, (24, 24));
        assert_eq!(
            input.normal.objects.get(&(0, 0)).unwrap(),
            &WarehouseObjectType::Wall,
        );

        assert_eq!(input.doublewide.robot_start, (48, 24));
        assert_eq!(
            input.normal.objects.get(&(0, 0)).unwrap(),
            &WarehouseObjectType::Wall,
        );
    }
}
