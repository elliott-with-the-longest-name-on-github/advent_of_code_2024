use std::collections::HashMap;

use crate::day06::{Input, ParsedMap};

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
    let mut blockages_by_x = HashMap::new();
    let mut blockages_by_y = HashMap::new();
    let mut start_x = 0;
    let mut start_y = 0;
    let y_len: u32 = input.lines().count().try_into().unwrap();
    let x_len: u32 = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let y: u32 = y.try_into().unwrap();
            let x: u32 = x.try_into().unwrap();
            if (c == '#') {
                blockages_by_x.entry(x).or_insert_with(Vec::new).push(y);
                blockages_by_y.entry(y).or_insert_with(Vec::new).push(x);
            } else if (c == '^') {
                start_x = x;
                start_y = y;
            }
        }
    }

    ParsedMap {
        blockages_by_x,
        blockages_by_y,
        start_x,
        start_y,
        x_len,
        y_len,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let by_x_test = input.blockages_by_x.get(&0).unwrap().first().unwrap();
        assert_eq!(by_x_test, &7);

        let by_y_test = input.blockages_by_y.get(&0).unwrap().first().unwrap();
        assert_eq!(by_y_test, &36);

        assert_eq!(input.start_x, 89);
        assert_eq!(input.start_y, 53);
    }
}
