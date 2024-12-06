use std::collections::HashMap;

use crate::day06::{Grid, GridPoint, Input};

const EXAMPLE_INPUT: &str = include_str!("../../input/06/example.txt");
const INPUT: &str = include_str!("../../input/06/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let y: i32 = y.try_into().unwrap();
            let x: i32 = x.try_into().unwrap();
            let mut is_blockage = false;
            if (c == '#') {
                is_blockage = true;
            } else if (c == '^') {
                start_x = x;
                start_y = y;
            }
            grid.insert((x, y), GridPoint { x, y, is_blockage });
        }
    }

    Grid {
        grid,
        start_point: GridPoint {
            x: start_x,
            y: start_y,
            is_blockage: false,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        assert_eq!(input.start_point.x, 89);
        assert_eq!(input.start_point.y, 53);
    }
}
