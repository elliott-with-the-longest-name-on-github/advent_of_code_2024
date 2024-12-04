use std::collections::HashMap;

use crate::day04::{Grid, Input};

const EXAMPLE_INPUT: &str = include_str!("../../input/04/example.txt");
const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let mut map = HashMap::new();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut x_size: i32 = 0;
    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let chars_len = chars.len() as i32;
        if (x_size == 0) {
            x_size = chars_len;
        } else if (x_size != chars_len) {
            panic!("Found a line of unequal length. Established line length: {x_size}, found line length: {chars_len}");
        }
        for (x, char) in line.chars().enumerate() {
            let key = format!("{x},{y}");
            map.insert(key, char);
        }
    }

    Grid {
        x_size,
        y_size: lines.len() as i32,
        map,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        assert_eq!(input.x_size, 140);
        assert_eq!(input.y_size, 140);

        let first = input.map.get("0,0").unwrap();
        assert_eq!(first, &'A');

        let last = input.map.get("139,139").unwrap();
        assert_eq!(last, &'X');
    }
}
