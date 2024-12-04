use crate::day04::{Input, Output};
use std::{collections::HashMap, fs::DirEntry};

use super::Direction;

pub fn solve(input: &Input) -> Output {
    let mut xmas_count = 0;
    let word = vec!['X', 'M', 'A', 'S'];
    for y in 0..input.y_size {
        for x in 0..input.x_size {
            xmas_count += find_word_count(&word, &input.map, x, y);
        }
    }

    xmas_count.into()
}

fn get_char(map: &HashMap<String, char>, x: i32, y: i32) -> Option<&char> {
    let key = format!("{},{}", x, y);
    map.get(&key)
}

fn find_word_count(word: &Vec<char>, map: &HashMap<String, char>, x: i32, y: i32) -> i32 {
    let first_letter = word.get(0).expect("Got a 0-length word");
    let char = get_char(map, x, y).expect(&format!("Somehow went out of bounds: {x},{y}"));
    if (char != first_letter) {
        return 0;
    }
    let mut word_count = 0;
    for search_y in y - 1..y + 2 {
        for search_x in x - 1..x + 2 {
            if y == search_y && x == search_x {
                continue;
            }

            let x_direction = get_direction(x, search_x);
            let y_direction = get_direction(y, search_y);

            let is_word = follow_word(&word[1..], map, x, y, &x_direction, &y_direction);
            if is_word {
                word_count += 1;
            }
        }
    }

    word_count
}

fn follow_word(
    word: &[char],
    map: &HashMap<String, char>,
    word_start_x: i32,
    word_start_y: i32,
    x_direction: &Direction,
    y_direction: &Direction,
) -> bool {
    for (i, required_letter) in word.iter().enumerate() {
        let magnitude: i32 = (i + 1).try_into().unwrap();
        let x = get_endpoint(word_start_x, magnitude, x_direction);
        let y = get_endpoint(word_start_y, magnitude, y_direction);
        let char = match get_char(map, x, y) {
            None => return false,
            Some(c) => c,
        };
        if (char != required_letter) {
            return false;
        }
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
