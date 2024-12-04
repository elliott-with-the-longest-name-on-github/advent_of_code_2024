use crate::day04::{Input, Output};
use std::{collections::HashMap, fs::DirEntry};

use super::Direction;

pub fn solve(input: &Input) -> Output {
    let mut x_mas_count = 0;
    let word = vec!['X', 'M', 'A', 'S'];
    for y in 0..input.y_size {
        for x in 0..input.x_size {
            x_mas_count += find_x_mas_count(&input.map, x, y);
        }
    }

    x_mas_count.into()
}

fn get_char(map: &HashMap<String, char>, x: i32, y: i32) -> Option<&char> {
    let key = format!("{},{}", x, y);
    map.get(&key)
}

fn find_x_mas_count(map: &HashMap<String, char>, x: i32, y: i32) -> i32 {
    let char = get_char(map, x, y).expect(&format!("Somehow went out of bounds: {x},{y}"));
    if (char != &'A') {
        return 0;
    }
    let mut word_count = 0;
    let search_coord_pairs = vec![
        ((x - 1, y - 1), (x + 1, y + 1)),
        ((x + 1, y - 1), (x - 1, y + 1)),
        ((x - 1, y + 1), (x + 1, y - 1)),
        ((x + 1, y + 1), (x - 1, y - 1)),
    ];
    for ((start_x, start_y), (end_x, end_y)) in search_coord_pairs {
        let is_word = is_mas(map, start_x, start_y, end_x, end_y);
        if is_word {
            word_count += 1;
        }
        if (word_count > 1) {
            return 1;
        }
    }

    // There can only be one X-MAS per A, so if we didn't return from the loop we know we didn't find an X
    0
}

fn is_mas(map: &HashMap<String, char>, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> bool {
    let first_char = match get_char(map, start_x, start_y) {
        None => return false,
        Some(c) => c,
    };
    if (first_char != &'M') {
        return false;
    }

    let last_char = match get_char(map, end_x, end_y) {
        None => return false,
        Some(c) => c,
    };
    if (last_char != &'S') {
        return false;
    }

    true
}

fn get_endpoint(start_point: i32, magnitude: i32, direction: &Direction) -> i32 {
    let direction_multiplier: i32 = match direction {
        Direction::None => 0,
        Direction::Increase => 1,
        Direction::Decrease => -1,
    };

    start_point + (magnitude * direction_multiplier)
}

fn get_direction(start_point: i32, end_point: i32) -> Direction {
    match end_point {
        n if n < start_point => Direction::Decrease,
        n if n > start_point => Direction::Increase,
        _ => Direction::None,
    }
}
